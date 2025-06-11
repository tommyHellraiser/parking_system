-- Table for storing each car's details
DROP TABLE IF EXISTS car_details;
CREATE TABLE car_details (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    license_plate VARCHAR(20) NOT NULL UNIQUE,
    car_color VARCHAR(30) NOT NULL,
    make VARCHAR(50) NOT NULL,
    model VARCHAR(50) NOT NULL,
    year INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Table for storing hourly rate tiers
DROP TABLE IF EXISTS hourly_rates;
CREATE TABLE hourly_rates (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    `name` VARCHAR(50) NOT NULL,
    rate DECIMAL(10,2) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

insert into hourly_rates(`name`, rate)
VALUES("Default houylr rate", 3.2);

-- Table for parking lots
DROP TABLE IF EXISTS parking_lots;
CREATE TABLE parking_lots (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    lot_name VARCHAR(100) NOT NULL,
    address VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Table for parking shifts
DROP TABLE IF EXISTS parking_shifts;
CREATE TABLE parking_shifts (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    car_ID INT NOT NULL,
    lot_ID INT NOT NULL,
    rate_ID INT NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY CONSTRAINT fk_car_id FOREIGN KEY (car_ID) REFERENCES car_details(ID),
    CONSTRAINT fk_lot FOREIGN KEY (lot_ID) REFERENCES parking_lots(lot_ID),
    CONSTRAINT fk_rate FOREIGN KEY (rate_ID) REFERENCES hourly_rates(rate_ID)
);

-- Table for payments
DROP TABLE IF EXISTS payments;
CREATE TABLE payments (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    shift_ID INT NOT NULL,
    amount DECIMAL(10,2) NOT NULL,
    payment_method ENUM('Cash', 'Card', 'Digital') NOT NULL,
    payment_time DATETIME NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN CONSTRAINT KEY fk_shift FOREIGN KEY (shift_ID) REFERENCES parking_shifts(ID)
);