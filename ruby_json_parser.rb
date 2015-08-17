
class JSONParser
  def initialize
    @data = File.read("/Users/Matt/projects/ruby/hearthstone/public/data/AllSets.json").chars
    @i = 0
  end

  def parse_key(key='')
    # fast-forward to the beginning of the key string.
    @i += 1 until @data[@i - 1] == '"'
    until @data[@i] == '"'
      key += @data[@i]
      @i += 1
    end
    @i += 2 # skip trailing quotation and colon
    return key
  end

  def parse_object(nested_level = 1, object={})
    until nested_level == 0
      object[parse_key] = parse
      until @data[@i] != ','
        @i += 1 # moves on to the next key in the object
        object[parse_key] = parse
      end

      if @data[@i] == '}'
        nested_level -= 1
      end
    end

    return object
  end

  def parse_array(array=[], nested_level = 1)
    @i += 1 # fast forward
    until nested_level == 0
      if @data[@i] == ']'
        nested_level -= 1
      else
        array << parse
        if @data[@i] == ']'
          nested_level -= 1
        else
          @i += @data[@i + 1] == ',' ? 2 : 1
        end
      end
    end
    @i += 1 # skip close bracket
    array
  end

  def char_escaped?
    @data[@i] == "\\"
  end

  def convert_escaped_character
    case @data[@i]
    when 'n' then "\n"
    when 'a' then "\a"
    when 'e' then "\e"
    when 'r' then "\r"
    when 's' then "\s"
    when '"' then "\""
    else
      @data[@i]
    end
  end

  def parse_string(str='', substring=false)
    @i += 1 if @data[@i] == "\""
    loop do
      char = @data[@i]
      break if char == "\""

      if char_escaped?
        @i += 1
        str += convert_escaped_character
      else
        str += char
      end
      @i += 1
    end
    @i += 1 if @data[@i] == "\""
    return str
  end

  def parse_number(n='')
    while ('0'..'9').include?(@data[@i])
      n += @data[@i]
      @i += 1
    end
    n.to_i
  end

  def parse_boolean(b=true)
    @data[@i] == 't' ? [@i += 4, b] : [@i += 5, !b]
  end

  def parse(container = nil)
    if @data[@i] == '{'
      parse_object
    elsif @data[@i] == '"'
      parse_string
    elsif ('0'..'9').include?(@data[@i])
      parse_number
    elsif @data[@i] == '['
      parse_array
    elsif @data[@i] == 'n'
      return i + 4, nil
    elsif @data[@i] == 'f' || @data[@i] == 't'
      _, bool = parse_boolean
      return bool
    end
  end
end

parser = JSONParser.new
json = parser.parse
