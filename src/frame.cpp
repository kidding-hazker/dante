#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <string>
using namespace std;

class Frame {
    private:
        int x, y, w, h;
        string title;
    public:
        Frame() {}
        ~Frame() {}
};
