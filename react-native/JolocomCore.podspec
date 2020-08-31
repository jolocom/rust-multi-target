require "json"

package = JSON.parse(File.read(File.join(__dir__, "package.json")))

Pod::Spec.new do |s|
  s.name         = "JolocomCore"
  s.summary      = package["description"]
  s.version      = package["version"]
  s.description  = <<-DESC
                  Rust based crypto functions used by the jolocom SDK
                   DESC
  s.homepage     = "https://github.com/jolocom/rust-multi-target"
  s.license      = "Apache-2.0"
  s.author       = { "Jolocom Dev" => "dev@jolocom.io" }
  s.platform     = :ios, "9.0"
  s.source       = { :git => "https://github.com/jolocom/rust-multi-target.git", :tag => "master" }
  s.source_files  = "ios/*.{h,m,swift}"
  s.requires_arc = true
  s.vendored_libraries = 'ios/libkeriox_wrapper.a'

  s.dependency "React"
end

