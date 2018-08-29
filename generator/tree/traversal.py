from collections import namedtuple, deque
from .errors import CircularReferencing
from .nodes.references import Reference, TypeReference


class _Traversal(object):
    def __init__(self, tree):
        self._root = tree.root

    @staticmethod
    def children(node):
        return [c for c in node.children if not isinstance(c, Reference)] + \
               [r.node for r in node.children if isinstance(r, TypeReference)]

    def iterate(self):
        raise RuntimeError("Derived classes must reimplement iterate()")


class BfsTraversal(_Traversal):
    def __init__(self, *args, **kwargs):
        super(BfsTraversal, self).__init__(*args, **kwargs)

    def iterate(self):
        Attr = namedtuple("Attr", ["distance"])

        queue = deque([(self._root, 0)])
        processed = set()
        while len(queue) != 0:
            node, distance = queue.popleft()
            if node in processed:
                continue
            yield node, Attr(distance=distance)
            for child in _Traversal.children(node):
                if child not in processed:
                    queue.append((child, distance + 1))
            processed.add(node)


class DfsTraversal(_Traversal):
    _PROCESS_NODE_EARLY = 0
    _PROCESS_NODE_LATE = 1

    def __init__(self, *args, **kwargs):
        super(DfsTraversal, self).__init__(*args, **kwargs)

    def _iterate(self):
        State = namedtuple("State", ["node", "processed"])
        Attr = namedtuple("Attr", [])
        stack = [State(node=self._root, processed=False)]
        discovered = set()
        processed = set()

        while len(stack) != 0:
            node, is_processed = stack.pop()
            if not is_processed:
                if node in processed:
                    continue

                yield self._PROCESS_NODE_EARLY, node, Attr()
                discovered.add(node)
                stack.append(State(node=node, processed=True))

                for child in reversed(_Traversal.children(node)):
                    if child not in discovered and child not in processed:
                        stack.append(State(node=child, processed=False))
                    elif child not in processed:
                        raise CircularReferencing(node, child)
            else:
                yield self._PROCESS_NODE_LATE, node, Attr()
                processed.add(node)

    def iterate(self):
        for event, node, attr in self._iterate():
            if event == self._PROCESS_NODE_EARLY:
                yield node, attr

    def dependency_order(self):
        for event, node, attr in self._iterate():
            if event == self._PROCESS_NODE_LATE:
                yield node, attr
