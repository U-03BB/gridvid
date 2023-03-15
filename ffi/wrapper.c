/*
This code was adapted from lieff's Mini MP4 embeddable MP4 mux/demux library
under Creative Commons Zero v1.0 Universal license.

Author: lieff
Repo:   https://github.com/lieff/minimp4
Source: https://github.com/lieff/minimp4/blob/master/minimp4_test.c
*/

#ifdef _WIN32
#include <sys/types.h>
#include <stddef.h>
typedef size_t ssize_t;
#endif

#define MINIMP4_IMPLEMENTATION
#include "minimp4.h"

static ssize_t get_nal_size(uint8_t *buf, ssize_t size)
{
    ssize_t pos = 3;
    while ((size - pos) > 3)
    {
        if (buf[pos] == 0 && buf[pos + 1] == 0 && buf[pos + 2] == 1)
            return pos;
        if (buf[pos] == 0 && buf[pos + 1] == 0 && buf[pos + 2] == 0 && buf[pos + 3] == 1)
            return pos;
        pos++;
    }
    return size;
}

static int write_callback(int64_t offset, const void *buffer, size_t size, void *token)
{
    FILE *f = (FILE*)token;
    fseek(f, offset, SEEK_SET);
    return fwrite(buffer, 1, size, f) != size;
}

void mux_mp4(char *filename, uint8_t *h264_buf, ssize_t h264_size, int width, int height, int fps) {
    FILE *file_ptr;
    file_ptr = fopen(filename, "wb");

    MP4E_mux_t *mux;
    mux = MP4E_open(0, 0, file_ptr, write_callback);

    mp4_h26x_writer_t mp4wr;
    if (MP4E_STATUS_OK != mp4_h26x_write_init(&mp4wr, mux, width, height, 0))
    {
        printf("error: mp4_h26x_write_init failed\n");
        exit(1);
    }

        while (h264_size > 0)
    {
        ssize_t nal_size = get_nal_size(h264_buf, h264_size);
        if (nal_size < 4)
        {
            h264_buf  += 1;
            h264_size -= 1;
            continue;
        }

        if (MP4E_STATUS_OK != mp4_h26x_write_nal(&mp4wr, h264_buf, nal_size, 90000 / fps))
        {
            printf("error: mp4_h26x_write_nal failed\n");
            exit(1);
        }
        h264_buf  += nal_size;
        h264_size -= nal_size;


    }
    MP4E_close(mux);
    mp4_h26x_write_close(&mp4wr);
    fclose(file_ptr);
}
