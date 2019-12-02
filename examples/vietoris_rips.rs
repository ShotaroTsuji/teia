use std::fmt::Write;
use std::io;
use std::io::BufWriter;
use std::fs::File;
use teia::Persistence;
use teia::vietoris_rips::{DistanceMatrix, enumerate_simplices};
use teia::simplex::Simplex;
use teia::complex::Complex;
use teia::indexed_vec::IndexedVec;
use teia::z2vector::{Z2Chain, Z2VectorVec};
use teia::z2reduce::Z2ColumnReduce;
use teia::pair::Pair;
use nalgebra::DVector;

#[derive(Debug, Clone)]
struct FillStyle {
    color: String,
    opacity: f64,
}

#[derive(Debug, Clone)]
struct StrokeStyle {
    color: String,
    width: f64,
}

#[derive(Debug, Clone, Copy)]
struct DrawSpec {
    scale: f64,
    size: f64,
}

impl DrawSpec {
    fn map(&self, coord: f64) -> f64 {
        self.scale*coord + self.size/2.0
    }

    fn map_scale(&self, w: f64) -> f64 {
        w * self.scale
    }
}

fn emit_svg_points(points: &[DVector<f64>], indices: &[&[usize]], ps: f64, style: FillStyle, spec: DrawSpec) -> String {
    let mut buf = String::new();

    writeln!(&mut buf, r#"<g fill="{}" fill-opacity="{}">"#, style.color, style.opacity).unwrap();

    for index in indices.iter() {
        let index = index[0].clone();
        let x = spec.map(points[index][0]);
        let y = spec.map(points[index][1]);

        writeln!(&mut buf, r#"<circle cx="{:.4}" cy="{:.4}" r="{:.4}" />"#, x, y, spec.map_scale(ps)).unwrap();
    }

    writeln!(&mut buf, "</g>").unwrap();
    buf
}

fn emit_svg_edges(points: &[DVector<f64>], edges: &[&[usize]], style: StrokeStyle, spec: DrawSpec) -> String {
    let mut buf = String::new();

    writeln!(&mut buf, r#"<g stroke="{}" stroke-width="{:.4}">"#, style.color, spec.map_scale(style.width)).unwrap();

    for e in edges.iter() {
        let x1 = spec.map(points[e[0]][0]);
        let y1 = spec.map(points[e[0]][1]);
        let x2 = spec.map(points[e[1]][0]);
        let y2 = spec.map(points[e[1]][1]);
        writeln!(&mut buf, r#"<line x1="{:.4}" y1="{:.4}" x2="{:.4}" y2="{:.4}" />"#, x1, y1, x2, y2).unwrap();
    }

    writeln!(&mut buf, "</g>").unwrap();
    buf
}

fn emit_svg_triangles(points: &[DVector<f64>], triangles: &[&[usize]], style: FillStyle, spec: DrawSpec) -> String {
    let mut buf = String::new();

    writeln!(&mut buf, r#"<g fill="{}" fill-opacity="{}">"#, style.color, style.opacity).unwrap();

    for t in triangles.iter() {
        let x1 = spec.map(points[t[0]][0]);
        let y1 = spec.map(points[t[0]][1]);
        let x2 = spec.map(points[t[1]][0]);
        let y2 = spec.map(points[t[1]][1]);
        let x3 = spec.map(points[t[2]][0]);
        let y3 = spec.map(points[t[2]][1]);
        writeln!(&mut buf, r#"<polygon points="{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}" />"#, x1, y1, x2, y2, x3, y3).unwrap();
    }

    writeln!(&mut buf, "</g>").unwrap();
    buf
}

fn emit_vr_svg<W: io::Write>(w: &mut W, points: &[DVector<f64>], pairs: &[(Vec<usize>, f64)], max_index: usize) -> io::Result<()> {
    let mut vertices: Vec<&[usize]> = Vec::new();
    let mut edges: Vec<&[usize]> = Vec::new();
    let mut triangles: Vec<&[usize]> = Vec::new();

    for index in 0..max_index {
        if pairs[index].0.len() == 1 {
            vertices.push(&pairs[index].0[..]);
        } else if pairs[index].0.len() == 2 {
            edges.push(&pairs[index].0[..]);
        } else if pairs[index].0.len() == 3 {
            triangles.push(&pairs[index].0[..]);
        }
    }

    let spec = DrawSpec { scale: 250.0, size: 1000.0 };
    let points_svg = emit_svg_points(points, &vertices[..], 0.05, FillStyle { color: "black".into(), opacity: 1.0 }, spec);
    let edges_svg = emit_svg_edges(points, &edges[..], StrokeStyle { color: "black".into(), width: 0.01 }, spec);
    let triangles_svg = emit_svg_triangles(points, &triangles[..], FillStyle { color: "#191970".into(), opacity: 0.2 }, spec);

    writeln!(w, r#"<?xml version="1.0" standalone="no"?>"#)?;
    writeln!(w, r#"<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">"#)?;
    writeln!(w, r#"<svg viewBox="0 0 1000 1000" xmlns="http://www.w3.org/2000/svg" version="1.1">"#)?;
    writeln!(w, "{}", triangles_svg)?;
    writeln!(w, "{}", edges_svg)?;
    writeln!(w, "{}", points_svg)?;
    writeln!(w, "</svg>")?;

    Ok(())
}

fn emit_balls_svg<W: io::Write>(w: &mut W, points: &[DVector<f64>], pairs: &[(Vec<usize>, f64)], max_index: usize) -> io::Result<()> {
    let mut vertices: Vec<&[usize]> = Vec::new();

    for index in 0..max_index {
        if pairs[index].0.len() == 1 {
            vertices.push(&pairs[index].0[..]);
        }
    }

    let spec = DrawSpec { scale: 250.0, size: 1000.0 };
    let points_svg = emit_svg_points(points, &vertices[..], 0.05, FillStyle { color: "black".into(), opacity: 1.0 }, spec);
    let balls_svg = emit_svg_points(points, &vertices[..], pairs[max_index].1/2.0, FillStyle { color: "#FFB7C5".into(), opacity: 0.5 }, spec);

    writeln!(w, r#"<?xml version="1.0" standalone="no"?>"#)?;
    writeln!(w, r#"<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">"#)?;
    writeln!(w, r#"<svg viewBox="0 0 1000 1000" xmlns="http://www.w3.org/2000/svg" version="1.1">"#)?;
    writeln!(w, "{}", balls_svg)?;
    writeln!(w, "{}", points_svg)?;
    writeln!(w, "</svg>")?;

    Ok(())
}

fn generate_circle(n: usize) -> Vec<DVector<f64>> {
    let mut points = Vec::with_capacity(n);

    for i in 0..n {
        let t = i as f64 / n as f64;
        let t = t * 2.0 * std::f64::consts::PI;
        let v = DVector::from_vec(vec![t.cos(), t.sin()]);
        points.push(v);
    }

    points
}

fn main() {
    let circle = generate_circle(12);

    let dist = DistanceMatrix::from_fn(circle.len(), |i, j| {
        let a = circle[i].clone() - &circle[j];
        a.norm()
    });

    for i in 0..circle.len() {
        for j in 0..circle.len() {
            eprint!("{:.3} ", dist.get(i, j).unwrap());
        }
        eprintln!("");
    }

    let mut pairs = Vec::new();

    for q in 0..3 {
        let mut q_pairs = enumerate_simplices(circle.len(), q, &dist);
        pairs.append(&mut q_pairs);
    }

    pairs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for (index, (simp, filt)) in pairs.iter().enumerate() {
        eprintln!("{}:\t{:?} -> {:.3}", index, simp, filt);
    }

    eprintln!("## {} simplices", pairs.len());

    for index in 0..pairs.len() {
        let name = format!("vr2/vr-comp-{}-{:.3}.svg", index, pairs[index].1);
        let f = File::create(name).unwrap();
        let mut w = BufWriter::new(f);
        emit_vr_svg(&mut w, &circle, &pairs, index+1).unwrap();

        let name = format!("vr2/vr-ball-{}-{:.3}.svg", index, pairs[index].1);
        let f = File::create(name).unwrap();
        let mut w = BufWriter::new(f);
        emit_balls_svg(&mut w, &circle, &pairs, index).unwrap();
    }

    let mut comp: Complex<IndexedVec<_>, _> = Complex::new();
    for (simp, _) in pairs.iter() {
        comp.push(Simplex::new(simp.to_vec())).unwrap();
    }

    for chain in comp.boundaries::<Z2VectorVec>() {
        eprintln!("{:?}", chain);
    }

    let reduce =
        Z2ColumnReduce::<Z2Chain<Z2VectorVec>>::from_complex_with(&comp, |index, image| {
            Z2Chain::new(index, image)
        })
        .unwrap();

    println!("## cycles");
    for c in reduce.cycles() {
        println!("{:?}", c);
    }
    println!("");

    println!("## Z2Pair");
    for (pers, _chain) in Pair::new(&reduce, reduce.cycles()) {
        let index = pers.0;
        let dim = pairs[index].0.len()-1;
        match pers {
            Persistence(b, Some(d)) => {
                println!("{} {} {} {:.4} {:.4}", dim, b, d, pairs[b].1, pairs[d].1);
            },
            Persistence(b, None) => {
                println!("{} {} N {:.4} inf", dim, b, pairs[b].1);
            },
        }
    }
}
