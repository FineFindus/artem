#!/bin/bash
# This script can be used to easily update when the test results should change
# That can happen even when to image is just slightly different, for example a single char is different.

echo "Building release"
cargo build --release

echo "Creating text files"

echo "Creating output file without extra arguments"
cargo run --release images/standard_test_img.png -o standard_test_img/standard_test_img.txt

echo "Creating output file with border"
cargo run --release images/standard_test_img.png --border -o standard_test_img/standard_test_img_border.txt

echo "Creating output file with outline and border"
cargo run --release images/standard_test_img.png --border --outline -o standard_test_img/standard_test_img_border_outline.txt

echo "Creating output file with outline"
cargo run --release images/standard_test_img.png --outline -o standard_test_img/standard_test_img_outline.txt

echo "Creating output file with outline and hysteresis"
cargo run --release images/standard_test_img.png --outline --hysteresis -o standard_test_img/standard_test_img_outline_hysteresis.txt


echo "Creating html files"

echo "Creating .html output file without extra arguments"
cargo run --release images/standard_test_img.png -o standard_test_img/standard_test_img.html

echo "Creating .html output file with background color"
cargo run --release images/standard_test_img.png --background -o standard_test_img/standard_test_img_background.html

echo "Creating .html output file with border"
cargo run --release images/standard_test_img.png --border -o standard_test_img/standard_test_img_border.html

echo "Creating .html output file with outline and border"
cargo run --release images/standard_test_img.png --border --outline -o standard_test_img/standard_test_img_border_outline.html

echo "Creating .html output file with outline"
cargo run --release images/standard_test_img.png --outline -o standard_test_img/standard_test_img_outline.html

echo "Creating .html output file with outline and hysteresis"
cargo run --release images/standard_test_img.png --outline  --hysteresis -o standard_test_img/standard_test_img_outline_hysteresis.html
