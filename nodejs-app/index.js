const express = require('express');
const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(express.json());

// Sample songs data
let songs = [
  { id: 1, title: "Bohemian Rhapsody", artist: "Queen", year: 1975 },
  { id: 2, title: "Stairway to Heaven", artist: "Led Zeppelin", year: 1971 },
  { id: 3, title: "Hotel California", artist: "Eagles", year: 1976 }
];

// Routes
app.get('/', (req, res) => {
  res.json({ message: 'Welcome to Song List API (Node.js)' });
});

app.get('/songs', (req, res) => {
  res.json(songs);
});

app.get('/songs/:id', (req, res) => {
  const song = songs.find(s => s.id === parseInt(req.params.id));
  if (!song) {
    return res.status(404).json({ error: 'Song not found' });
  }
  res.json(song);
});

app.post('/songs', (req, res) => {
  const { title, artist, year } = req.body;
  if (!title || !artist) {
    return res.status(400).json({ error: 'Title and artist are required' });
  }
  
  const newSong = {
    id: songs.length > 0 ? Math.max(...songs.map(s => s.id)) + 1 : 1,
    title,
    artist,
    year: year || new Date().getFullYear()
  };
  
  songs.push(newSong);
  res.status(201).json(newSong);
});

app.delete('/songs/:id', (req, res) => {
  const index = songs.findIndex(s => s.id === parseInt(req.params.id));
  if (index === -1) {
    return res.status(404).json({ error: 'Song not found' });
  }
  
  songs.splice(index, 1);
  res.status(204).send();
});

app.listen(PORT, '0.0.0.0', () => {
  console.log(`Node.js Song List API server is running on port ${PORT}`);
});
