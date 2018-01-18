from conans import ConanFile, CMake, tools


class FlatdataConan(ConanFile):
    generators = "cmake"
    name = "flatdata"
    version = "0.1"
    license = "Apache 2.0"
    description = "Zero-overhead serialization and deserialization of memory mapped archives."
    url = "https://github.com/heremaps/flatdata"
    settings = "os", "compiler", "build_type", "arch"
    generators = "cmake"
    exports_sources = "*"

    def build(self):
        cmake = CMake(self)
        cmake.configure()
        cmake.build()

    def package(self):
        self.copy("*.h", dst="include", src="flatdata-cpp/include")
        self.copy("*flatdata.[lib|dll|so|dylib|a]", dst="lib", keep_path=False)

    def package_info(self):
        self.cpp_info.libs = ["flatdata"]
