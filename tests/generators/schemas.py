import os

def schemas_and_expectations(generator, extension):
    result = list()
    basedir = os.path.dirname(__file__)
    test_dir = os.path.normpath(os.path.join(basedir, '..', '..', 'test_cases'))
    for path, subdirs, files in os.walk(test_dir):
        for name in files:
            if os.path.splitext(name)[1] == '.flatdata':
                relpath = os.path.relpath(path, test_dir)
                test_relpath = os.path.join(relpath, name)
                expecation_path = os.path.join(basedir,
                                               generator + '_expectations',
                                               os.path.splitext(test_relpath)[0]+'.' + extension)
                result.append((os.path.join(path, name), expecation_path))
    return result