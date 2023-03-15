#ifdef _WIN32
typedef size_t ssize_t;
#endif

#include "minimp4.h"

void mux_mp4(char *filename, uint8_t *h264_buf, ssize_t h264_size, int width, int height, int fps);
