# Sort images from different devices by exif metadata

| Argument  | Description                   | Example                    |
| --------- | ----------------------------- | -------------------------- |
| --out     | output directory              | sorted-files               |
| --folders | 1 or more folders with images | phone-images camera-images |

To sort all images (files will be copied rather than moved):

```sh
cargo run -- --out out-dir --folders phone-images camera-images
```

This will write all files to `out-dir` and prefix the file name with a timestamp. The timestamp is timezone adjusted.

- 20240217145540_original-file-name.jpg
- 20240217148546_rick-astley.jpg
- ...
