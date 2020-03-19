require 'sinatra/base'

class HelloWebServer < Sinatra::Base
  get '*' do
    "hello, you've hit #{env["REQUEST_PATH"]}\n"
  end
end
