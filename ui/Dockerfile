# Use the official Node.js image as the base
FROM node:20

# Set the working directory inside the container
WORKDIR /app

# Copy package.json and package-lock.json to the working directory
COPY package*.json ./

# Install the application dependencies
RUN npm ci

# Copy the entire application code to the working directory
COPY . .

# Build the Next.js application
RUN npm run build

# Expose the port on which the application will run (default is 3000 for Next.js)
EXPOSE 3000

# Start the application
CMD ["npm", "run", "start"]
