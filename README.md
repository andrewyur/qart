# QArt Code Generator
An implementation of [QArt Codes](https://research.swtch.com/qr/draw/) in rust.

<img alt="qr code with a pattern that looks like a cat" src="https://raw.githubusercontent.com/andrewyur/qart/master/mascot.png" width=400/>

## Installation (for github)
This repository is published as a crate on cargo, and can be installed with `cargo install qart`.
If you dont have cargo and are on windows, download the zipped binary in the releases section and unpack it.

## Usage
This crate can be used as both an executable and a dependency in cargo projects. For usage instructions, run  `qart help` or `./path/to/qart help` if the executable was downloaded. If installed as a dependency, the functions `qart::build` and `qart::preview` are exposed to the user.

## Tips
High contrast images work well, and drawings in ms paint work [particularly well](https://github.com/andrewyur/qart/blob/master/mascot2.png), although images with a bad light/dark balance often dont work at the highest qr code sizes.

The brightness threshold value is similar to exposure in photography, except inverted: a higher value means more pixels will be black.

These qr codes have a low error correction level to allow for more drawing room. So, depending on the code, scanners may need a bit more time and a clearer view to scan them compared to standard qr codes.

The way this crate manipulates the qr code appearance while still maintaining their functionality is by appending a url fragment after the supplied target text, so unfortunately if you want to encode just text, or any other data besides a url, there will be a massive string of numbers after the target data. Read more about the process [here](https://research.swtch.com/qart), where the creator of the process, Russ Cox (of Go fame), describes it in further detail. However, the article does not cover the technical details, and if you are looking to make your own implementation, I tried to document the process as well as practicality allowed, so feel free to take a look inside.

## Speed vs. Other Implementations
Because this is written in rust, and is multithreaded, this crate is an order of magnitude faster than other implementations of QArt codes.

<!--TODO: figure out how to benchmark the original implementation-->
I was able to benchmark all of the following except except [Russ Cox's original implementation](https://github.com/rsc/qr) because it was encompassed by a website, and the qr code functionality was difficult to access on its own.

The following were made to generate a v40 qr code with the source image used in the qr code shown above:
- https://github.com/dieforfree/qart4j:   14.16s
- https://github.com/7sDream/pyqart:      94.18s
- qart:                                   284.87ms

## A Note on the TODOs
If you take a look at the source code, you will see TODO comments scattered heavily throughout it. The only things left on the TODO list are restructuring and potential optimization. I am new to rust, so I am not sure to what extent the code is optimized by the compiler, so things like removing unused vector allocations and caching are on there. At this point, attempting to make (technical) optimizations is starting take a lot longer, and have diminishing returns (the last attempted optimization broke the code, and also made it slower so i decided it wasnt worth it). Since I don't really care if this crate is used by anyone else, continuing to work on todos is not going to be benefiting anyone much, so I have decided to call it here, and pivot to working on other projects.

## More Info 
Read more about it on my website: https://andrewyur.github.io/#p/qart-encoder

## Related Links
- https://research.swtch.com/field
- https://research.swtch.com/qart
- https://www.thonky.com/qr-code-tutorial/
- https://www.nayuki.io/page/creating-a-qr-code-step-by-step
