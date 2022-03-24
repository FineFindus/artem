complete -c artem -s c -l characters -d 'Change the characters that are used to display the image.The first character should have the highest \'density\' and the last should have the least (probably a space).A lower detail map is recommend for smaller images.' -r -f
complete -c artem -s s -l size -d 'Change the size of the output image.The minimum size is 20, the maximum 230. Values outside of the range will beignored and changed to the nearest usable value. This argument is conflicting with --width and --height' -r -f
complete -c artem -l ratio -d 'Change the ratio between height and width, since Ascii chars are a bit higher than long.The default value is 0.43, min is 0 and max 2. It is not recommend to change this setting.' -r -f
complete -c artem -s o -l output -d 'Output file for non-colored ascii. If the output file is a plaintext file, no color will be used. The use color, either use a file with an .ansi extension, or an .html file, to convert the output to html.' -r -F
complete -c artem -l thread -d 'OutputNumber of threads used to convert the image. A larger number can lead to grater performance. Defaults to 4' -r -f
complete -c artem -l verbose -d 'Choose the verbosity of the logging level.' -r -f -a "{trace	,debug	,info	,warn	,error	}"
complete -c artem -l help -d 'Print help information'
complete -c artem -s V -l version -d 'Print version information'
complete -c artem -s h -l height -d 'Use the terminal maximum terminal height to display the image.This argument is conflicting with --size and --width '
complete -c artem -s w -l width -d 'Use the terminal maximum terminal height to display the image.This argument is conflicting with --size and --height '
complete -c artem -l flipX -d 'Flip the image along the X axis'
complete -c artem -l flipY -d 'Flip the image along the Y axis'
complete -c artem -l invert -d 'Inverts the characters used for the image, so light characters will as dark ones. Can be useful if the image has a dark background.'
complete -c artem -l background -d 'Sets the background of the ascii as the color. This will be ignored if the terminal does not support truecolor. This argument is mutually exclusive with the no-color argument.'
complete -c artem -l border -d 'Adds a decorative border surrounding the ascii image. This will make the image overall a bit smaller, since it respects the user given size.'
complete -c artem -l no-color -d 'Do not use color when printing the image to the terminal.'
