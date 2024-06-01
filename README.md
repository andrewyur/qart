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
- Image Encoder
  - ~~process target image~~ can wait
    - shrink image down to dimensions of target qr code
    - black and white
      - called thresholding
      - no function in image crate, will have to automate, or provide slider to user
  - encoding in url fragment
  - encoding in error correction modules
  - encoding in data modules (?)

  
## Long Term Itinerary
- benchmarking
- Compile to WASM and host on gh-pages
- Pixel editor
- color selector
