-- The width of the generated image in pixels (required)
width = 1024;
-- The height of the generated image in pixels (required)
height = 1024;
-- Make the generated image in grayscale
--grayscale = false;
-- Make the colors on the generated image inverted
--invert = false;
-- Apply a gaussian blur to the generated image
--blur = false;
-- Brighten the generated image
--contrast_adj = 0;
-- Brighten the pixels of the generated image
--brighten = 0;
-- Flip the generated image vertically
--flip_v = false;
-- Flip the generated image horizontally
--flip_h = false;
-- Rotate the image by n degrees, which must be a multiple of 90 degrees,
-- and <= 360 degrees.
--rotate = 0;
-- The name of the output file. Format is png.
--output = "output.png";

function red()
  return bit32.band(x, x);
end

function blue()
  return 0;
end

function green()
  return "n";
end

function alpha()
  return 255;
end
