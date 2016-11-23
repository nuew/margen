-- The width of the generated image in pixels (required)
width = 1024;
-- The height of the generated image in pixels (required)
height = 1024;
-- The name of the output file. Format is png.
output = "sstv.png";

function square(n)
  return n*n
end

function red()
  return math.sqrt(2 * (square(x - width/2) * square(y - height/2)));
end

function blue()
  return math.sqrt(
    bit32.bor(square(x - width/2), square(y - height/2)) *
    bit32.band(square(x - width/2), square(y - height/2))
  );
end

function green()
  return math.sqrt(2 * bit32.band(square(x - width/2), square(y - height/2)));
end

function alpha()
  return 255;
end
