# THREE FINGER DRAG
OSX users will miss the three-finger-drag feature when switch to Linux. With that feature, you can easily select text or drag something around. On Linux, there are two common options to achieve this:
1. [mtrack](https://github.com/p2rkw/xf86-input-mtrack)

set the following options in xorg.conf and you get three-finger-drag
```
Option "SwipeUpButton" "1"
Option "SwipeDownButton" "1"
Option "SwipeLeftButton" "1"
Option "SwipeRightButton" "1"
```

The problems of mtrack are:    
* not actively maintained
* less "smooth" than libinput
* not the default driver for trackpad of Manjaro or Ubuntu, and have some compatibility issues with GDM on my machine


2. [libinput-gestures](https://github.com/bulletmark/libinput-gestures)

libinput-gestures do not natively support three-finger-drag. You need to use this fork [three-finger-drag fork](https://github.com/daveriedstra/libinput-gestures/tree/three-finger-drag) which is mentioned in this [libinput-gestures three-finger-drag issue](https://github.com/bulletmark/libinput-gestures/issues/10#issuecomment-441459797). As the author of libinput-gestures [replied](https://github.com/bulletmark/libinput-gestures/issues/10#issuecomment-247980222), libinput-gestures won't support three-finger-drag due to "To implement this would be messy, require significant processing overhead, and is discordant with the current design.". 

so if you use the [three-finger-drag fork](https://github.com/daveriedstra/libinput-gestures/tree/three-finger-drag) of libinput-gestures:
* you can't use new features of libinput-gestures because the two repos can not be merged easily
* extra configuration is needed
    ```
    gesture swipebegin all 3 xdotool mousedown 1
    gesture swipeend all 3 xdotool mouseup 1
    gesture swipeupdate all 3 xdotool mousemove_relative -- x y
    ```

# INSTALLATION

So with this tool, you can use three-finger-drag with libinput(the underlying tool libinput-gestures based on). You probably will use it with libinput-gestures side by side to add three-finger-drag support.

## From Source
```
1. clone the repo
2. install libinput(you probably have it already)
3. install xdotool(which includes libxdo)
4. cargo build --release
5. copy target/release/libinput-three-finger-drag to somewhere
6. disable 3 finger swipe gesture in libinput-gestures, see below
7. run libinput-three-finger-drag and check whether it works
8. make libinput-three-finger-drag auto-start
```
## Download binary
```
1. download precompiled binary from release page
2. disable 3 finger swipe gesture in libinput-gestures, see below
3. run libinput-three-finger-drag and check whether it works
4. make libinput-three-finger-drag auto-start
```

## Disable 3 finger swipe gesture in libinput-gestures
Modify libinput-gestures config file /etc/libinput-gestures.conf or ~/.config/libinput-gestures.conf. 
Add finger_count 4 to essentially disable 3 finger swipe.

change
``` 
gesture swipe up  xdotool key super+Page_Down 
```
to
```
gesture swipe up  4  xdotool key super+Page_Down
```

# HOW IT WORKS
Just like libinput-gestures. Fork
```
libinput debug-events
```
and read the output, which is trackpad "raw events". Filter out events of GESTURE_SWIPE_BEGIN, GESTURE_SWIPE_UPDATE and GESTURE_SWIPE_END. Simulate mouse_down on BEGIN, mouse_relative_move on UPDATE and mouse_up on END with libxdo. These three mouse action simulate a three-finger-drag. Call libxdo is more efficient than fork xdotool like libinput-gestures does. Especially when processing GESTURE_SWIPE_UPDATE events which is more frequent than BEGIN and END. 
