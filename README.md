# Image to MakeCode Arcade

This is a a CLI tool to convert images (and videos) into a format that is usable in MakeCode Arcade. This project was written in rust.

# Usage
## Single Images

To use this tool with still images you will need to run the executable with the following arguments
```
image_to_makecode --size <SIZE> --img <IMAGE> --output <OUTPUT> --colormap <COLORMAP>
```

| size     | Provide the desired size of the output image in the format WIDTHxHEIGHT |
| -------- | ----------------------------------------------------------------------- |
| img      | Provide the path to the image (most common image formats are supported) |
| output   | Provide the path for the output file (usually a txt file)               |
| colormap | Provide the desired color palette.                                      |
Included color palettes:
* arcade
* matte
* pastel
* sweet
* poke
* adventure
* diy
* adafruit
* stilllife
* steampunk
* grayscale

Once you have generated the output, copy it and paste it into your MakeCode JavaScript like so:
```
let mySprite = sprites.create(<PASTE HERE>, SpriteKind.Player)
```
### Example:
```
image_to_makecode --size 32x32 --img cat.jpg --output img.txt --colormap arcade
```
## Videos/Gifs
This tool also works on videos and gifs (to turn into sprite animations). This has the potential to cause a lot of lag in MakeCode Arcade if you have a high resolution and/or a lot of frames.

Use in the same way as single images except provide the path to a folder of frames. It is recommended to use FFMPEG to convert a video/gif to a folder of frames.

Once you have generated the output, copy it and paste it into your MakeCode JavaScript like so:
```
animation.runImageAnimation(mySprite, [<PASTE HERE>], 500, false)
```

### Example
```
mkdir frames
ffmpeg -i cat.gif frames/frame_%0d.png
image_to_makecode --size 32x32 --img frames --output anim.txt --colormap arcade
```
# Adding Custom Color Palettes
This tool allows you to add your own color palettes. Edit the file called colors.txt and add your own palette to the end of the file like so:

```
<PALETTE NAME (NO SPACES)>:
1
#HEX COLOR
2
#HEX COLOR
3
#HEX COLOR
4
#HEX COLOR
5
#HEX COLOR
6
#HEX COLOR
7
#HEX COLOR
8
#HEX COLOR
9
#HEX COLOR
10
#HEX COLOR
11
#HEX COLOR
12
#HEX COLOR
13
#HEX COLOR
14
#HEX COLOR
15
#HEX COLOR
```
Look at the existing palettes for examples. The easiest way to do this is to enter the name of your color palette, then copy and paste directly from MakeCode Arcade (don't worry about empty lines).
