import os

NUM_TEST_CASES = 25

def schemas_and_expectations(generator, extension):
    """
    Retrieves list of test cases filenames and generates corresponding expectation filenames
    generator: one of the supported generator: `rust`, `cpp`, `flatdata`, `dot`, `rust`, `go
    extension: extension of the expectation files for this generator, e.g. `.h` for `cpp`
    """
    result = list()
    basedir = os.path.dirname(__file__)
    test_dir = os.path.normpath(os.path.join(
        basedir, '..', '..', '..', 'test_cases'))
    for path, _subdirs, files in os.walk(test_dir):
        for name in files:
            if os.path.splitext(name)[1] == '.flatdata':
                relpath = os.path.relpath(path, test_dir)
                test_relpath = os.path.join(relpath, name)
                expectation_filename = os.path.splitext(
                    test_relpath)[0]+'.' + extension
                expecation_path = os.path.join(
                    basedir, generator + '_expectations', expectation_filename)
                result.append((os.path.join(path, name), expecation_path))
    assert len(result) == NUM_TEST_CASES, "Did not find expected number of test cases in " + test_dir
    return result
