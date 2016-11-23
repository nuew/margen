-- The width of the generated image in pixels (required)
width = 1024;
-- The height of the generated image in pixels (required)
height = 1024;
-- The name of the output file. Format is png.
output = "sierpinski.png";

function red()
  if x and y then
    return bit32.band((x % y), (y %x));
  else
    return 0;
  end
end

function blue()
  if x and y then
    return (x % y) + (y % x);
  else
    return 0;
  end
end

function green()
  if x and y then
    return bit32.bor((x % y), (y % x));
  else
    return 0;
  end
end

function alpha()
  return 255;
end
