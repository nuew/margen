-- The width of the generated image in pixels (required)
width = 1024;
-- The height of the generated image in pixels (required)
height = 1024;
-- The name of the output file. Format is png.
output = "mandelbrot.png";

cn = -1;
cx = -1;
cy = -1;

function mandelbrot(k)
  local n = 0;

  if cy ~= y or cx ~= x then
    local xbb = xb+7
    for x=xb,xbb < width and xbb or width-1 do
      bits = bits + bits
      local Zr, Zi, Zrq, Ziq = 0.0, 0.0, 0.0, 0.0
      local Cr = x * wscale - 1.5
      for i=1,m do
        local Zri = Zr*Zi
        Zr = Zrq - Ziq + Cr
        Zi = Zri + Zri + Ci
        Zrq = Zr*Zr
        Ziq = Zi*Zi
        if Zrq + Ziq > limit2 then
          bits = bits + 1
          break
        end
                                                                                end
                                                                                    end
                                                                                        if xbb >= width then
                                                                                                for x=width,xbb do bits = bits + bits + 1 end
                                                                                                    end

    cn = n;
    cx = x;
    cy = y;
  else
    n = cn;
  end

  return 255 - n;
end

function red()
  return mandelbrot();
end

function blue()
  return mandelbrot();
end

function green()
  return mandelbrot(); 
end

function alpha()
  return 255;
end
