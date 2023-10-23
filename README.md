# Quick Sort

## Quick Sort

Algoritmo de ordenação que utiliza a estratégia de dividir para conquistar. A estratégia consiste em rearranjar as chaves de modo que as chaves "menores" precedam as chaves "maiores". Em seguida o quicksort ordena as duas sublistas de chaves menores e maiores recursivamente até que a lista completa se encontre ordenada. Os passos são:

1. Escolha um elemento da lista, denominado pivô;
2. Particiona: rearranje a lista de forma que todos os elementos anteriores ao pivô sejam menores que ele, e todos os elementos posteriores ao pivô sejam maiores que ele. Ao fim do processo o pivô estará em sua posição final e haverá duas sublista não ordenadas. Essa operação é denominada partição;
3. Recursivamente ordene a sublista dos elementos menores e a sublista dos elementos maiores;
4. A base da recursão são as listas de tamanho zero ou um, que estão sempre ordenadas.
5. O processo é finito, pois a cada iteração pelo menos um elemento fica em sua posição final e não precisa ser processado novamente.
6. O processo não precisa de espaço adicional para a realização da ordenação.

## Complexidade do Algoritmo

O algoritmo de ordenação Quick Sort, na notação Big-O, possui complexidade de tempo de O(n log n) no melhor caso, O(n log n) no caso médio e O(n²) no pior caso.


