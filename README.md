# QArt Code Generator

implementation of [QArt Code generator](https://research.swtch.com/qr/draw/) in rust

## Resources
- https://research.swtch.com/field
- https://research.swtch.com/qart
- https://www.thonky.com/qr-code-tutorial/
- https://www.nayuki.io/page/creating-a-qr-code-step-by-step

## Other Implementations
- https://github.com/rsc/qr
- https://github.com/dieforfree/qart4j
- https://github.com/7sDream/pyqart

## Current Itinerary
- get working completely
  - data bits need to be interleaved completely before interleaving of error bits
  - will need two separate iters probably
- Get randomness working properly
- Image Encoder
  - process target image
    - shrink image down to dimensions of target qr code
    - black and white
      - called thresholding
      - no function in image crate, will have to automate, or provide slider to user

  
## Long Term Itinerary
- benchmarking & optimization
- Compile to WASM and host on gh-pages
- Pixel editor
- color selector
- publish to crates.io
  - will need a cmd line parser + all the other things
