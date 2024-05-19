# QArt Code Generator

implementation of [QArt Code generator](https://research.swtch.com/qr/draw/) in rust

## Resources
- https://research.swtch.com/field
- https://research.swtch.com/qart
- https://www.thonky.com/qr-code-tutorial/

## Current Itinerary
- Working QR code generator
  - Reed Solomon Encoding
    - faster + cleaner way to find generator polynomial
  - Final Structuring
  - Matrix Placement
  - Data Masking
  - Format & Version Info
  - Export to Image
- Image Encoder
  
## Long Term Itinerary
- Compile to WASM and host on gh-pages
