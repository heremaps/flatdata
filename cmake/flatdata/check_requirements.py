import pkg_resources
import sys

dependencies = open(sys.argv[1]).readlines()
pkg_resources.require(dependencies)
