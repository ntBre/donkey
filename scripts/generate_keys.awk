#!/usr/bin/awk -f

BEGIN {
	split("abcdefghijklmnopqrstuvwxyz", chars, "")
	for (c in chars) {
		char = toupper(chars[c])
		printf "%s = raylib_sys::KeyboardKey_KEY_%s as i32,\n", char, char
	}
}
