from collections import defaultdict, deque

def topological_sort(graph):
    in_degree = defaultdict(int)
    all_nodes = set(graph.keys())

    for node in graph:
        all_nodes.update(graph[node])
        for neighbor in graph[node]:
            in_degree[neighbor] += 1

    queue = deque([node for node in all_nodes if in_degree[node] == 0])
    result = []

    while queue:
        node = queue.popleft()
        result.append(node)

        for neighbor in graph.get(node, []):
            in_degree[neighbor] -= 1
            if in_degree[neighbor] == 0:
                queue.append(neighbor)

    if len(result) != len(all_nodes):
        raise ValueError("Graph has a cycle")

    return result
