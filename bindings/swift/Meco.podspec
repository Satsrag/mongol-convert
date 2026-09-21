Pod::Spec.new do |s|
  s.name         = 'Meco'
  s.version      = '0.6.1'
  s.summary      = 'Mongolian Encoding Converter — Rust core via UniFFI.'
  s.description  = 'Convert between Mongolian encodings (Zvvnmod, Z52, Menk-shape/letter, Delehi). '\
                   'Backed by the shared Rust meco core, verified byte-exact against the original Java.'
  s.homepage     = 'https://github.com/Satsrag/mongol-convert'
  s.license      = { :type => 'Apache-2.0' }
  s.author       = 'zvvnmod'
  s.platform     = :ios, '13.0'
  s.swift_version = '5.9'

  # The release archive contains the framework and the generated Swift wrapper.
  s.source                = { :http => "https://github.com/Satsrag/mongol-convert/releases/download/v#{s.version}/MecoSwift.xcframework.zip" }
  s.vendored_frameworks   = 'MecoSwift.xcframework'
  s.source_files          = ['sw/*.swift', 'Sources/Meco/*.swift']
end
