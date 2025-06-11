--  Dropping tables
DROP TABLE IF EXISTS payments;
DROP TABLE IF EXISTS parking_shifts;
DROP TABLE IF EXISTS parking_lots;
DROP TABLE IF EXISTS hourly_rates;
DROP TABLE IF EXISTS car_details;

--  Creating tables
-- Table for storing each car's details
CREATE TABLE car_details (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    license_plate VARCHAR(20) NOT NULL UNIQUE,
    car_color VARCHAR(30) NOT NULL,
    make VARCHAR(50) NOT NULL,
    model VARCHAR(50) NOT NULL,
    `year` INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Table for storing hourly rate tiers
CREATE TABLE hourly_rates (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    `name` VARCHAR(50) NOT NULL,
    rate DECIMAL(10,2) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Table for parking lots
CREATE TABLE parking_lots (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    lot_name VARCHAR(100) NOT NULL,
    `address` VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Table for parking shifts
CREATE TABLE parking_shifts (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    car_ID INT NOT NULL,
    lot_ID INT NOT NULL,
    rate_ID INT NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME DEFAULT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY CONSTRAINT fk_car_id FOREIGN KEY (car_ID) REFERENCES car_details(ID),
    CONSTRAINT fk_lot FOREIGN KEY (lot_ID) REFERENCES parking_lots(lot_ID),
    CONSTRAINT fk_rate FOREIGN KEY (rate_ID) REFERENCES hourly_rates(rate_ID)
);

-- Table for payments
CREATE TABLE payments (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    shift_ID INT NOT NULL,
    amount DECIMAL(10,2) NOT NULL,
    method ENUM('Cash', 'Card', 'Digital') NOT NULL,
    `time` DATETIME NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN CONSTRAINT KEY fk_shift FOREIGN KEY (shift_ID) REFERENCES parking_shifts(ID)
);

--  Inserting initial data
INSERT INTO hourly_rates(`name`, rate)
VALUES("Default houurly rate", 3.2);
