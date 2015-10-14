
extern crate time;

/// A high precision timer.
#[derive(Copy,Clone)]
pub struct Timer
{
    /// The last marked time (in nanoseconds, with an unspecified epoch).
    last_marked: u64,
    
    /// The delta time (in seconds) since the timer was last marked, or the timer
    /// was created if it has not been explicitly marked yet.
    delta: f64,
}

impl Timer
{
    /// Creates a new timer and marks the current time.
    ///
    pub fn new() -> Timer
    {
        Timer {
            last_marked: time::precise_time_ns(),
            delta: 0.0,
        }
    }
    
    /// Marks the current time and returns the delta time since the last marking.
    /// If the timer has not been marked yet, the delta time is the time since the object
    /// was created.
    ///
    /// The delta time is in seconds.
    pub fn mark(&mut self) -> f64
    {
        let current_time = time::precise_time_ns();
        
        let delta_ns = (current_time - self.last_marked) as f64;
        
        self.delta = delta_ns/1000000000.0;
        self.last_marked = current_time;
        
        self.delta
    }
    
    /// Gets the time (in seconds) since the timer was last marked (or the object was
    /// created if it has not been explicitly marked yet).
    pub fn get_delta(&self) -> f64
    {
        self.delta
    }
}
