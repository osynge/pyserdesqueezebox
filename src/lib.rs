use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass]
#[derive(Clone, Debug)]
struct Player {
    pub name: String,
    pub uuid: String,
    pub id: String,
    pub ip: String,
    pub model: String,
    pub firmware_version: String,
    pub signal_strength: u8,
    pub play_state: String,
}



#[pymethods]
impl Player {

     #[getter]
     fn get_signal_strength(&self) -> PyResult<u8> {
        Ok(self.signal_strength)
     }
}


#[pyfunction]
fn double(x: usize) ->  PyResult<Vec<Player>>  {
    
    
    let mut p1 = Player{
        name: String::from("Hello, world!"),
        uuid: String::from("Hello, world!"),
 id: String::from("Hello, world!"),
 ip: String::from("Hello, world!"),
 model: String::from("Hello, world!"),
 firmware_version: String::from("Hello, world!"),
 signal_strength: 2,
 play_state: String::from("Hello, world!"),        
        };
    let bill : Vec<Player> = [p1].to_vec();
    Ok(bill)
}



#[pymodule]
fn pyserdesqueezebox(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(double)).unwrap();
    m.add_class::<Player>()?;
    Ok(())
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
