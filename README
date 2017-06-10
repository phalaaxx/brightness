brightness
--

Usage:
	brightness /sys/class/backlight/.../brightness 187 +2
	brightness /sys/class/backlight/.../brightness 187 -2

brightness accepts three positional arguments:
 - name of kernel interface
 - max value of brightness (as shown by reading max_brightness)
 - a positive or negative value to add to current brigtness

compile
--

	rustc brightness.rs

Note: the program may need root permissions to adjust kernel value, thus it may be necessary to run it with sudo.

