"""

Copyright 2022 Wilhelm Ågren

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

File created: 2022-09-23
Last updated: 2022-09-23
"""
import matplotlib.pyplot as plt
from collections import defaultdict

data = defaultdict(list)
with open('metrics.txt') as f:
    for line in f.readlines():
        matrix_dim, row_ms, col_ms = line.rstrip().split(' ')
        data['matrix_dim'].append(float(matrix_dim))
        data['row_ms'].append(float(row_ms))
        data['col_ms'].append(float(col_ms))

plt.plot(data['matrix_dim'], data['row_ms'], label='row', linewidth=1.5, linestyle='--')
plt.plot(data['matrix_dim'], data['col_ms'], label='col', linewidth=1.5)
plt.title('row-vs-column-looping')
plt.xlabel('2D matrix dimensionality')
plt.ylabel('average looping time [ms]')
plt.legend()
plt.show()

