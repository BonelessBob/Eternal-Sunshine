## Notes and Stuff

### Idea for the project:
The idea is to have an app where you can input an image, and through image recognition the app will generate music.
This allows me to create different sounds for different images.
* For mountain ranges I was thinking of using edge detection to find where the mountains meet the sky, and with this data create a melody
  * Maybe even more abstractely use edge detection to decide some stuff regarding the synth/reverb/whatever device, some other algorithms to decide other stuff, and then create different "models" that specifies what algorithm decides what
* I'll have to be able to generate sound. How I don't quite know yet... Nannou has generative sound capabilities [Talk on how this can be done](https://www.youtube.com/watch?v=JPFv3adyLB4)

At some point I will have to figure out how to stream the data from nannou to dioxus. The nannou mentioned something about OSC or whatever it was. Also TCP I guess.

### ToDo:
I'll begin with the mountain ranges as I'm currently looking at one. Later I can think about how to switch algorithm depending on what the image contains...
  