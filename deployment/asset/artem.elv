
use builtin;
use str;

set edit:completion:arg-completer[artem] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'artem'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'artem'= {
            cand -c 'Change the characters that are used to display the image.The first character should have the highest ''density'' and the last should have the least (probably a space).A lower detail map is recommend for smaller images.'
            cand --characters 'Change the characters that are used to display the image.The first character should have the highest ''density'' and the last should have the least (probably a space).A lower detail map is recommend for smaller images.'
            cand -s 'Change the size of the output image.The minimum size is 20, the maximum 230. Values outside of the range will beignored and changed to the nearest usable value. This argument is conflicting with --width and --height'
            cand --size 'Change the size of the output image.The minimum size is 20, the maximum 230. Values outside of the range will beignored and changed to the nearest usable value. This argument is conflicting with --width and --height'
            cand --ratio 'Change the ratio between height and width, since Ascii chars are a bit higher than long.The default value is 0.43, min is 0 and max 2. It is not recommend to change this setting.'
            cand -o 'Output file for non-colored ascii. If the output file is a plaintext file, no color will be used. The use color, either use a file with an .ansi extension, or an .html file, to convert the output to html.'
            cand --output 'Output file for non-colored ascii. If the output file is a plaintext file, no color will be used. The use color, either use a file with an .ansi extension, or an .html file, to convert the output to html.'
            cand --thread 'OutputNumber of threads used to convert the image. A larger number can lead to grater performance. Defaults to 4'
            cand --verbose 'Choose the verbosity of the logging level.'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand -h 'Use the terminal maximum terminal height to display the image.This argument is conflicting with --size and --width '
            cand --height 'Use the terminal maximum terminal height to display the image.This argument is conflicting with --size and --width '
            cand -w 'Use the terminal maximum terminal height to display the image.This argument is conflicting with --size and --height '
            cand --width 'Use the terminal maximum terminal height to display the image.This argument is conflicting with --size and --height '
            cand --flipX 'Flip the image along the X axis'
            cand --flipY 'Flip the image along the Y axis'
            cand --invert 'Inverts the characters used for the image, so light characters will as dark ones. Can be useful if the image has a dark background.'
            cand --background 'Sets the background of the ascii as the color. This will be ignored if the terminal does not support truecolor. This argument is mutually exclusive with the no-color argument.'
            cand --border 'Adds a decorative border surrounding the ascii image. This will make the image overall a bit smaller, since it respects the user given size.'
            cand --no-color 'Do not use color when printing the image to the terminal.'
        }
    ]
    $completions[$command]
}
