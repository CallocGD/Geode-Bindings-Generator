# Geode-Bindings-Generator
A New commandline progam that hopefully will soon replace the currently used bidnings scripts in the [bindings repo](https://github.com/geode-sdk/bindings)
it planned from the start that people should having to go out of thier way to install both node-js and python and many people might already be like 
that's a waste of diskspace and I agree. Nobody should have to have had already installed java on top of that to use ghidra. The point here
is that the installion process in order to use things is too much and I felt a simpler solution was required to solve this problem. 
so I am taking both my previously made python script and geode's node-js script and I am merging them together in rust. 

# Notes
- I am not currently done writing this script however many key functions are complete and I would say I have about (75%) of this code done (as long as it works correctly). Please know that I probbaly suck
at this language and I desire help from outside parties for help with optimizing or reorganizing the code once the first
release is confirmed to be working. The requirements for using this cli are limited and that is intentional and you will
have to download whatever the newest release of rust is in order to help out.
- You can find copies of the old scripts in the `__references__` folder. This was intentional for anyone who wanted to help out with porting these scripts to rust right away.
- There is currently no binrary release yet. A workflow is planned in the future for fixing that.


 


