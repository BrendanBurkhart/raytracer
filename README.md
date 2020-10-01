# Raytracer
A ray tracer written in Rust to learn about ray tracing, and computer graphics more generally. This project is the successor to an earlier project of mine, https://github.com/brendanburkhart/go-raytracer, a simpler ray tracer written in Go.

Currently supports loading models from the Wavefront [.OBJ format](https://en.wikipedia.org/wiki/Wavefront_.obj_file), as well as materials and textures from the companion .MTL format.

Examples:
![Classic teapot on checkered background](/examples/teapot/teapot.png "Classic teapot on checkered background")
![3D FIRST Robotics Competition Logo](/examples/first-logo/first-logo.png "#D FIRST Robotics Competition Logo")

## Building

Once you have Rust installed, and this repository cloned, simply run `cargo build` or `cargo build --release` in the root in order to build. The resulting executable will be created in `/target/debug/` or `/target/release/`. 

## Usage

Given a configuration file (described next), run the program with the path to the config file as the first argument in order to render an image. The configuration files used to produce the example images above are given at [/examples/teapot/config.json](/examples/teapot/config.json) and [/examples/first-logo/config.json](/examples/first-logo/config.json).

The configuration file must contain JSON with:
 - "lightingFile" giving a path to a scene lighting file,
 - "modelFile" giving a path to an .OBJ model file,
 - "maximumReflections" specifying a limit on the number of times a ray can reflect,
 - "camera" containing:
   - "viewWidth" specifying the width of the view port to be rendered
   - "position" specifying the camera position as a vector (`{"x": <x>, "y": <y>, "z": <z>}`),
   - "target" specifying where the camera is pointing, also as a vector,
   - "roll" specifying the angle in degrees to rotate the camera about the axis formed from position to target,
   - "focalLength" specifying the focal length to be used
 - "output" containing:
   - "imageWidth" specifying the pixel width of the output image,
   - "imageHeight" specifying the pixel height of the output image,
   - "imageFile" specifying the path to save the output image at.

The scene lighting file should be JSON containing an array under the "lights" key of object containing:
 - "position" specifying the position of the light source as a vector as above,
 - "specular" specifying the specular color component of the light as an array (`[1.0, 1.0, 1.0]` would be white light),
 - "diffuse" specifying the specular color component of the light, also as an array,
 - "ambient" specifying the contribution of the light source to the ambient light of the scene.
