```bash
Point cloud stats:
  Number of points: 20064
  Min bound: (-73.352, -17.787, -1.888)
  Max bound: (32.581, 7.805, 8.817)
  Scale: 19796.967
Morton3D kernel execution time: 0.010 ms

Morton codes (first 10):
  Point 0: code = 0x10CA88E6B2449009, index = 0
  Point 1: code = 0x10C7C31D74D1C472, index = 1
  Point 2: code = 0x10C7C27FDA0B72CC, index = 2
  Point 3: code = 0x10CA88E6B2449009, index = 3
  Point 4: code = 0x10CA88E6B2449009, index = 4
  Point 5: code = 0x10C7C35088508C97, index = 5
  Point 6: code = 0x10C7C27E679D8FE0, index = 6
  Point 7: code = 0x10CA88E6B2449009, index = 7
  Point 8: code = 0x10CA88E6B2449009, index = 8
  Point 9: code = 0x10C7C274902589C5, index = 9

Sorting on CPU...
Sorted! First 10 codes:
  [0] Code: 0x0092D165121229A0, Original Index: 8930
  [1] Code: 0x0288CB5612EBE701, Original Index: 13281
  [2] Code: 0x0288CB743FC6111B, Original Index: 8954
  [3] Code: 0x028F5F55E715CFA8, Original Index: 7853
  [4] Code: 0x029B2EB9C78FC318, Original Index: 12158
  [5] Code: 0x02C041FC09B68168, Original Index: 10042
  [6] Code: 0x02C043FD5EA6D3CC, Original Index: 12202
  [7] Code: 0x02C05AB6232E73A8, Original Index: 8957
  [8] Code: 0x02C0CA9B124DC40A, Original Index: 13285
  [9] Code: 0x02C0CABB15538690, Original Index: 8958
Sort kernel execution time: 0.007 ms
First point: Point3 { x: -73.352, y: 7.805, z: 0.859 }
Second point: Point3 { x: -39.929, y: -1.494, z: 0.397 }
Third point: Point3 { x: -39.998, y: -1.293, z: 0.435 }

Starting simple search...
Search kernel execution time: 1.445 ms
Search completed.
Point 0: Found neighbor index 0, DistSq = 0.000000
Point 1: Found neighbor index 1, DistSq = 0.000000
Point 2: Found neighbor index 2, DistSq = 0.000000
Point 3: Found neighbor index 3, DistSq = 0.000000
Point 4: Found neighbor index 4, DistSq = 0.000000
Point 5: Found neighbor index 5, DistSq = 0.000000
Point 6: Found neighbor index 6, DistSq = 0.000000
Point 7: Found neighbor index 7, DistSq = 0.000000
Point 8: Found neighbor index 8, DistSq = 0.000000
Point 9: Found neighbor index 9, DistSq = 0.000000
```

# Search 500 points
```bash
Point cloud stats:
  Number of points: 20064
  Min bound: (-73.352, -17.787, -1.888)
  Max bound: (32.581, 7.805, 8.817)
  Scale: 19796.967
Morton3D kernel execution time: 0.009 ms

Morton codes (first 10):
  Point 0: code = 0x10CA88E6B2449009, index = 0
  Point 1: code = 0x10C7C31D74D1C472, index = 1
  Point 2: code = 0x10C7C27FDA0B72CC, index = 2
  Point 3: code = 0x10CA88E6B2449009, index = 3
  Point 4: code = 0x10CA88E6B2449009, index = 4
  Point 5: code = 0x10C7C35088508C97, index = 5
  Point 6: code = 0x10C7C27E679D8FE0, index = 6
  Point 7: code = 0x10CA88E6B2449009, index = 7
  Point 8: code = 0x10CA88E6B2449009, index = 8
  Point 9: code = 0x10C7C274902589C5, index = 9

Sorting on CPU...
Sorted! First 10 codes:
  [0] Code: 0x0092D165121229A0, Original Index: 8930
  [1] Code: 0x0288CB5612EBE701, Original Index: 13281
  [2] Code: 0x0288CB743FC6111B, Original Index: 8954
  [3] Code: 0x028F5F55E715CFA8, Original Index: 7853
  [4] Code: 0x029B2EB9C78FC318, Original Index: 12158
  [5] Code: 0x02C041FC09B68168, Original Index: 10042
  [6] Code: 0x02C043FD5EA6D3CC, Original Index: 12202
  [7] Code: 0x02C05AB6232E73A8, Original Index: 8957
  [8] Code: 0x02C0CA9B124DC40A, Original Index: 13285
  [9] Code: 0x02C0CABB15538690, Original Index: 8958
Sort kernel execution time: 0.007 ms
First point: Point3 { x: -73.352, y: 7.805, z: 0.859 }
Second point: Point3 { x: -39.929, y: -1.494, z: 0.397 }
Third point: Point3 { x: -39.998, y: -1.293, z: 0.435 }
Number of query points: 500

Starting simple search...
Search kernel execution time: 0.755 ms
Search completed.
Search for 500 points completed.
Point 0: Found neighbor index 3400, DistSq = 0.000000
Point 1: Found neighbor index 2734, DistSq = 0.000000
Point 2: Found neighbor index 2725, DistSq = 0.000000
Point 3: Found neighbor index 3400, DistSq = 0.000000
Point 4: Found neighbor index 3400, DistSq = 0.000000
Point 5: Found neighbor index 2741, DistSq = 0.000000
Point 6: Found neighbor index 2724, DistSq = 0.000000
Point 7: Found neighbor index 3400, DistSq = 0.000000
Point 8: Found neighbor index 3400, DistSq = 0.000000
Point 9: Found neighbor index 2721, DistSq = 0.000000
```