// wrapper.h
#pragma once

// Tell RainmeterAPI to only give us the pure DLL‐exported prototypes
#define LIBRARY_EXPORTS

// Force the Windows headers to use the "W" (wide) APIs
#ifndef UNICODE
  #define UNICODE
#endif
#ifndef _UNICODE
  #define _UNICODE
#endif

// Speed up compilation a bit
#define WIN32_LEAN_AND_MEAN

#include <Windows.h>       // now GetModuleHandle → GetModuleHandleW
#include "sdk/API/RainmeterAPI.h"  // sees LIBRARY_EXPORTS, so no inline GetProcAddress code

