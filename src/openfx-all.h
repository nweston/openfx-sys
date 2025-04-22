// Combined header for OFX

// OFX is not cleanly divided into modules and many of the headers include
// each other, so if we run bindgen on each one separately we get many
// duplicate symbols.

// Rather than fiddling with allow/deny lists, include everything we need
// here and let the C preprocessor sort things out.

#include <stdbool.h>

#include <ofxImageEffect.h>
#include <ofxGPURender.h>
#include <ofxProgress.h>
#include <ofxTimeLine.h>
#include <ofxParametricParam.h>
#include <ofxDialog.h>
#include <ofxDrawSuite.h>
#include <ofxKeySyms.h>
