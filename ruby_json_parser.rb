require 'pry'
require 'json'
class JSONParser
  attr_reader :json

  def initialize
    @data = File.read("/Users/Matt/projects/ruby/hearthstone/public/data/AllSets.json").chars
    @accurate_json = JSON.load(@data.join)
    @i = 0
    @json = {}
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
        @i += 1
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

  def parse_string(str='', escaped_quote=false)
    until @data[@i + 1] == '"'
      str += @data[@i += 1]
    end
    @i += 2

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
    puts @data[@i]
    if @data[@i] == '{'
      puts 'object'
      parse_object
    elsif @data[@i] == '"'
      puts 'string'
      parse_string
    elsif ('0'..'9').include?(@data[@i])
      puts 'number'
      parse_number
    elsif @data[@i] == '['
      puts 'array'
      parse_array
    elsif @data[@i] == 'n'
      puts 'null'
      return i + 4, nil
    elsif @data[@i] == 'f' || @data[@i] == 't'
      puts 'boolean'
      _, bool = parse_boolean
      return bool
    elsif container.is_a?(Hash)

    elsif container.is_a?(Array)
    else
      binding.pry
    end
  end
end

parser = JSONParser.new
parser.parse
# key = parser
binding.pry
