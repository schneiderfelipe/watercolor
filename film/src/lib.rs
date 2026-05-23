use watercolor_sound as sound;

use watercolor_silent_film as silent_film;

pub trait Film: silent_film::SilentFilm + sound::Sound {}
impl<T: silent_film::SilentFilm + sound::Sound> Film for T {}
