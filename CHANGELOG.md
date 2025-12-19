## [0.5.1] Rustdoc patch
- Move definition of C GUID to lib.rs via windows-sys

## [0.5.0] vJoy update
- Update to vjoy version 2.2.2.0

## [0.4.1] Rustdoc patch
- Fixed: Broken docs link inside readme.

## [0.4.0] Windows 11 compatibility
- Changed: Most recent supported version of vJoy for Win 11 is 2.1.9.1.
- Changed: This version only supports up to 8 axes per device.

## [0.3.0] Updated vJoy version
- Added: Support for up to 4 continuous hat switches in the range of 0..360° with a 1/100° resolution.
- Changed: Max number of axes increased to 16
- Changed: Supported vJoy driver version updated to 2.2.2.1 from: https://github.com/njz3/vJoy/

## [0.2.0] 4-way hat switch support

## [0.1.2] Rustdoc patch
- Fixed: Removed unused and unincluded header files for GUID operations.
- Added: Hint that this lib is Windows-only

## [0.1.1] Rustdoc patch
- Fixed: Docs.rs tried to built the documentation on a linux machine and failed. Only target now is x86_64-pc-windows-msvc.
- Changed: Moved test to separate folder "tests".

## [0.1.0] Initial release