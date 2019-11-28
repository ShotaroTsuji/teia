use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct DistanceMatrix {
    /// the number of points
    size: usize,
    data: Vec<f64>,
}

impl DistanceMatrix {
    pub fn from_fn<F>(size: usize, mut f: F) -> DistanceMatrix
        where F: FnMut(usize, usize) -> f64
    {
        let mut data = Vec::with_capacity(size*(size+1)/2);

        for i in 0..size {
            for j in i..size {
                data.push(f(i, j));
            }
        }

        DistanceMatrix {
            size: size,
            data: data,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> Option<f64> {
        let (i, j) = if i > j { (j, i) } else { (i, j) };

        let pos = if i == 0 {
            j
        } else {
            self.size*i - (i-1)*i/2 + j - i
        };

        self.data.get(pos).cloned()
    }
}

fn filtration_value(vertices: &[usize], dist: &DistanceMatrix) -> f64 {
    assert!(vertices.len() > 0);
    if vertices.len() == 1 {
        0.0
    } else if vertices.len() == 2 {
        dist.get(vertices[0], vertices[1]).unwrap()
    } else {
        (0..vertices.len()).combinations(2)
            .map(|edge| {
                let e0 = vertices[edge[0]];
                let e1 = vertices[edge[1]];
                dist.get(e0, e1).unwrap()
            }).fold(0.0/0.0, f64::max)
    }
}

pub fn enumerate_simplices(n: usize, q: usize, dist: &DistanceMatrix) -> Vec<(Vec<usize>, f64)> {
    let mut pairs = (0..n).combinations(q+1)
        .map(|vertices| {
            let f = filtration_value(&vertices, dist);
            (vertices, f)
        }).collect::<Vec<(Vec<usize>, f64)>>();

    pairs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    pairs
}
