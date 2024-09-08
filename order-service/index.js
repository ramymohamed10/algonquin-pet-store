// index.js
const express = require('express');
const amqp = require('amqplib/callback_api');
const cors = require('cors');  // Import the CORS middleware

const app = express();
app.use(express.json());

// Enable CORS for all routes
app.use(cors());

const RABBITMQ_URL = 'amqp://localhost';

app.post('/orders', (req, res) => {
  const order = req.body;
  
  // Send order to RabbitMQ
  amqp.connect(RABBITMQ_URL, (err, conn) => {
    if (err) {
      return res.status(500).send('Error connecting to RabbitMQ');
    }
    conn.createChannel((err, channel) => {
      if (err) {
        return res.status(500).send('Error creating channel');
      }
      const queue = 'order_queue';
      const msg = JSON.stringify(order);
      
      channel.assertQueue(queue, { durable: false });
      channel.sendToQueue(queue, Buffer.from(msg));
      console.log("Sent order to queue:", msg);
      
      res.send('Order received');
    });
  });
});

const PORT = 3000;
app.listen(PORT, () => {
  console.log(`Order service is running on http://localhost:${PORT}`);
});
