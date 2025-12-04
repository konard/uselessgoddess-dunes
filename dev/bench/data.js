window.BENCHMARK_DATA = {
  "lastUpdate": 1764837247744,
  "repoUrl": "https://github.com/konard/uselessgoddess-dunes",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "68294279+uselessgoddess@users.noreply.github.com",
            "name": "uselessgoddess",
            "username": "uselessgoddess"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ea41f4eb0c86b9efe0cc74d78cbe487b40df3bc4",
          "message": "Merge pull request #25 from konard/issue-24-8b8684529f8d\n\nImprove benchmarks quality",
          "timestamp": "2025-12-03T23:08:51+03:00",
          "tree_id": "a2e46d1b5a34d59748376ec34b824bfccefc6b5a",
          "url": "https://github.com/uselessgoddess/dunes/commit/ea41f4eb0c86b9efe0cc74d78cbe487b40df3bc4"
        },
        "date": 1764792579621,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert + Remove (100 elements)",
            "value": 115.94,
            "range": "± 1.07",
            "unit": "M links/sec",
            "extra": "200 operations in 1725.07 ns/iter"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 81.63,
            "range": "± 3.18",
            "unit": "M links/sec",
            "extra": "2000 operations in 24499.43 ns/iter"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 61.48,
            "range": "± 6.12",
            "unit": "M links/sec",
            "extra": "100 operations in 1626.52 ns/iter"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 44.39,
            "range": "± 0.72",
            "unit": "M links/sec",
            "extra": "1000 operations in 22529.36 ns/iter"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 33.83,
            "range": "± 0.68",
            "unit": "M links/sec",
            "extra": "10000 operations in 295591.73 ns/iter"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 88.98,
            "range": "± 0.94",
            "unit": "M links/sec",
            "extra": "200 operations in 2247.69 ns/iter"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 58.38,
            "range": "± 0.81",
            "unit": "M links/sec",
            "extra": "2000 operations in 34260.18 ns/iter"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 25.77,
            "range": "± 0.39",
            "unit": "M links/sec",
            "extra": "20000 operations in 776042.38 ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "68294279+uselessgoddess@users.noreply.github.com",
            "name": "uselessgoddess",
            "username": "uselessgoddess"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1c8287987fe3f62e068a8f14ccd8ceebe724c295",
          "message": "Merge pull request #23 from konard/issue-22-2c1825bb9843\n\nAdd a complete implementation of ART (Adaptive Radix Tree)",
          "timestamp": "2025-12-03T23:33:42+03:00",
          "tree_id": "eb92beb5ddfb9ce4dbe54d689197550547371eb0",
          "url": "https://github.com/konard/uselessgoddess-dunes/commit/1c8287987fe3f62e068a8f14ccd8ceebe724c295"
        },
        "date": 1764831385630,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert Only (100 elements)",
            "value": 58.49,
            "range": "± 0.3",
            "unit": "M links/sec",
            "extra": "100 operations in 1709.60 ns/iter"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 42.97,
            "range": "± 0.14",
            "unit": "M links/sec",
            "extra": "1000 operations in 23273.00 ns/iter"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 32.24,
            "range": "± 0.08",
            "unit": "M links/sec",
            "extra": "10000 operations in 310210.00 ns/iter"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 85.2,
            "range": "± 0.13",
            "unit": "M links/sec",
            "extra": "200 operations in 2347.40 ns/iter"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 57.91,
            "range": "± 0.08",
            "unit": "M links/sec",
            "extra": "2000 operations in 34537.00 ns/iter"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 25.79,
            "range": "± 0.03",
            "unit": "M links/sec",
            "extra": "20000 operations in 775470.00 ns/iter"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 111.97,
            "range": "± 0.36",
            "unit": "M links/sec",
            "extra": "200 operations in 1786.20 ns/iter"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 85.08,
            "range": "± 0.15",
            "unit": "M links/sec",
            "extra": "2000 operations in 23507.00 ns/iter"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 65.21,
            "range": "± 0.12",
            "unit": "M links/sec",
            "extra": "100 operations in 1533.50 ns/iter"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 61.51,
            "range": "± 0.1",
            "unit": "M links/sec",
            "extra": "1000 operations in 16257.00 ns/iter"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 61.08,
            "range": "± 0.13",
            "unit": "M links/sec",
            "extra": "10000 operations in 163720.00 ns/iter"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 122.4,
            "range": "± 0.2",
            "unit": "M links/sec",
            "extra": "200 operations in 1634.00 ns/iter"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 117.41,
            "range": "± 0.21",
            "unit": "M links/sec",
            "extra": "2000 operations in 17035.00 ns/iter"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 116.77,
            "range": "± 0.21",
            "unit": "M links/sec",
            "extra": "20000 operations in 171270.00 ns/iter"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 113.01,
            "range": "± 0.25",
            "unit": "M links/sec",
            "extra": "200 operations in 1769.80 ns/iter"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 108.1,
            "range": "± 0.21",
            "unit": "M links/sec",
            "extra": "2000 operations in 18501.00 ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "68294279+uselessgoddess@users.noreply.github.com",
            "name": "uselessgoddess",
            "username": "uselessgoddess"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "133af21e3dbde116920531ef8c416a90a9ce9b06",
          "message": "Merge pull request #29 from konard/issue-28-397ada389b8b\n\nAdd custom benchmark viewer with tree comparison and unit toggles",
          "timestamp": "2025-12-04T10:44:10+03:00",
          "tree_id": "bde13027dd441c672a9926f9bc6c85428c18cc56",
          "url": "https://github.com/konard/uselessgoddess-dunes/commit/133af21e3dbde116920531ef8c416a90a9ce9b06"
        },
        "date": 1764835126629,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert Only (100 elements)",
            "value": 57.68,
            "range": "± 0.36",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=100 time=1733.70ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 42.37,
            "range": "± 0.39",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=1000 time=23602.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 32.5,
            "range": "± 0.23",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=10000 time=307670.00ns"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 87.72,
            "range": "± 0.48",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=2279.90ns"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 56.31,
            "range": "± 0.3",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=35517.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 26.5,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=754590.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 114.38,
            "range": "± 0.19",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=1748.50ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 81.87,
            "range": "± 0.81",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=24429.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 64.27,
            "range": "± 0.44",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=311210.00ns"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 68,
            "range": "± 0.14",
            "unit": "M links/sec",
            "extra": "tree=ART ops=100 time=1470.60ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 60.8,
            "range": "± 0.89",
            "unit": "M links/sec",
            "extra": "tree=ART ops=1000 time=16448.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 62.4,
            "range": "± 0.22",
            "unit": "M links/sec",
            "extra": "tree=ART ops=10000 time=160250.00ns"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 127.54,
            "range": "± 0.26",
            "unit": "M links/sec",
            "extra": "tree=ART ops=200 time=1568.10ns"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 120.98,
            "range": "± 0.08",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=16532.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 120.05,
            "range": "± 0.15",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=166600.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 119.68,
            "range": "± 0.07",
            "unit": "M links/sec",
            "extra": "tree=ART ops=200 time=1671.10ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 113.62,
            "range": "± 0.16",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=17602.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 112.66,
            "range": "± 0.16",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=177520.00ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "68294279+uselessgoddess@users.noreply.github.com",
            "name": "uselessgoddess",
            "username": "uselessgoddess"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a64d46aed3074e33ad0d7cc48c1a61f403f2a027",
          "message": "Merge pull request #31 from konard/issue-30-c40765cb9a35\n\nFix empty benchmark charts on github pages",
          "timestamp": "2025-12-04T11:15:06+03:00",
          "tree_id": "efb7e9c37993117a57890cc7eb9611b4f7efe8c8",
          "url": "https://github.com/konard/uselessgoddess-dunes/commit/a64d46aed3074e33ad0d7cc48c1a61f403f2a027"
        },
        "date": 1764837247501,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Insert Only (100 elements)",
            "value": 60.39,
            "range": "± 0.06",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=100 time=1656.00ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 43.25,
            "range": "± 0.2",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=1000 time=23119.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 31.73,
            "range": "± 0.14",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=10000 time=315120.00ns"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 87.98,
            "range": "± 0.39",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=2273.30ns"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 57.39,
            "range": "± 0.11",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=34851.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 26.45,
            "range": "± 0.02",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=756010.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 113.41,
            "range": "± 0.24",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=200 time=1763.50ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 83.89,
            "range": "± 0.32",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=2000 time=23842.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 62.68,
            "range": "± 0.22",
            "unit": "M links/sec",
            "extra": "tree=SBT ops=20000 time=319060.00ns"
          },
          {
            "name": "Insert Only (100 elements)",
            "value": 67.79,
            "range": "± 0.05",
            "unit": "M links/sec",
            "extra": "tree=ART ops=100 time=1475.10ns"
          },
          {
            "name": "Insert Only (1000 elements)",
            "value": 63.42,
            "range": "± 0.03",
            "unit": "M links/sec",
            "extra": "tree=ART ops=1000 time=15767.00ns"
          },
          {
            "name": "Insert Only (10000 elements)",
            "value": 62.75,
            "range": "± 0.31",
            "unit": "M links/sec",
            "extra": "tree=ART ops=10000 time=159350.00ns"
          },
          {
            "name": "Insert + Search (100 elements)",
            "value": 127.19,
            "range": "± 0.51",
            "unit": "M links/sec",
            "extra": "tree=ART ops=200 time=1572.50ns"
          },
          {
            "name": "Insert + Search (1000 elements)",
            "value": 120.71,
            "range": "± 0.11",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=16569.00ns"
          },
          {
            "name": "Insert + Search (10000 elements)",
            "value": 119.8,
            "range": "± 0.06",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=166940.00ns"
          },
          {
            "name": "Insert + Remove (100 elements)",
            "value": 119.1,
            "range": "± 0.22",
            "unit": "M links/sec",
            "extra": "tree=ART ops=200 time=1679.20ns"
          },
          {
            "name": "Insert + Remove (1000 elements)",
            "value": 113.11,
            "range": "± 0.06",
            "unit": "M links/sec",
            "extra": "tree=ART ops=2000 time=17682.00ns"
          },
          {
            "name": "Insert + Remove (10000 elements)",
            "value": 112.82,
            "range": "± 0.17",
            "unit": "M links/sec",
            "extra": "tree=ART ops=20000 time=177280.00ns"
          }
        ]
      }
    ]
  }
}