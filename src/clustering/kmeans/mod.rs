//! Kmeans -- Kmeans Algorithm in Rust
#![unstable]
extern crate matrixrs;
use matrixrs::{Matrix, ToMatrix};
use std::num::{zero};
use std::rand::{Rng, StdRng};
use std::rand::distributions::range::SampleRange;
use std::default::Default;

pub struct KMeans<T> {
	data: Matrix<T>,
}

impl<T:Primitive+SampleRange+Default> KMeans<T> {
	pub fn new(data: Matrix<T>) -> KMeans<T> {
		KMeans {
			data: data
		}
	}

	fn gen_center(points: &Matrix<T>) -> Matrix<T> {
		let mut rev_points = !points;

		rev_points.rows().map(
			|row| -> T {
				row.iter().fold(zero(),|acc: T, b| acc + *b) / rev_points.num_col
		}).to_matrix(1, points.col) 
	}

	fn assign_points(&self, centers: Matrix<T>) -> Vec<uint> {
		self.data.rows().map(|row| {	row	})}

		assignments = []
        for point in self.__matrix:
            shortest = float("inf")  #正无穷
            shortest_index = 0
            for i in range(len(centers)):
                val = self.__distance(point, centers[i])
                if val < shortest:
                   shortest = val
                   shortest_index = i

            assignments.append(shortest_index)
	}
	// pub fn update_centers(&self, assignments) {

	
	// }
    
        // new_means = collections.defaultdict(list)

        // centers = []
        // for assignment, point in zip(assignments, self.__matrix):
        //     new_means[assignment].append(point)

        // for points in new_means.values():
        //     centers.append(self.__point_avg(points))

        // return centers
    fn generate_k(&self, k: uint) -> Matrix<T> {
		//!找出所有点坐标中，最大的坐标和最小的坐标
    	   let mut ks = Matrix::new(k, self.data.col);
        
        for (idx, dim) in (!self.data).rows().enumerate() {
        	   let (max, min) = 
               dim.iter().map(|x| x.clone()).fold(
                (zero(), zero()),
                |acc, b| {
                    match acc {
        	 			(max, min) if b > max => (b, min),
        	 			(max, min) if b < min => (max, min),
        	 			_ => acc
                    }
                });

        	   for i in range(0, k) { ks.set(i, idx, StdRng::new().unwrap().gen_range(min, max)) }
        }

       ks
    }

    fn euclid_distance(ths: &Matrix<T>, rhs: &Matrix<T>) -> f64 {
        ths.iter().zip(rhs.iter()).fold(0f64, |acc, (a,b)| {
            acc + (NumCast::from(a).unwrap_or(0f64) - NumCast::from(b).unwrap_or(0f64)).powi(2)
    		// TODO fix the unwrap_or
        }).sqrt()
    }

    pub fn k_means(&self, k: uint) {
        let centers = self.generate_k(k);
    }
    //     assignments = self.__assign_points(centers)
    //     old_assignments = None
    //     while assignments != old_assignments:
    //         centers = self.__update_centers(assignments)
    //         old_assignments = assignments
    //         assignments = self.__assign_points(centers)
    //     return (zip(assignments, self.__matrix),centers)
    // }
}
