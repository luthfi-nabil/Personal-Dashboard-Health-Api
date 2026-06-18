CREATE DATABASE IF NOT EXISTS `health`
    CHARACTER SET utf8mb4
    COLLATE utf8mb4_uca1400_ai_ci;

USE `health`;

CREATE TABLE `insulin_assign` (
  `insulin_assign_id` char(36) NOT NULL,
  `insulin_item_id` char(36) NOT NULL,
  `batch_no` varchar(255) NOT NULL,
  `added_at` datetime NOT NULL,
  `notes` text DEFAULT NULL,
  `is_active` int(11) NOT NULL,
  `created_by` varchar(255) NOT NULL,
  PRIMARY KEY (`insulin_assign_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

CREATE TABLE `insulin_item` (
  `insulin_item_id` char(36) NOT NULL,
  `insulin_item_name` varchar(255) NOT NULL,
  `units` float NOT NULL,
  `uom` varchar(255) NOT NULL,
  `created_at` datetime NOT NULL,
  `notes` text DEFAULT NULL,
  `is_active` int(11) NOT NULL,
  `created_by` varchar(255) NOT NULL,
  PRIMARY KEY (`insulin_item_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

CREATE TABLE `insulin_usage` (
  `insulin_usage_id` char(36) NOT NULL,
  `insulin_assign_id` varchar(255) NOT NULL,
  `units` float NOT NULL,
  `administered_at` datetime NOT NULL,
  `notes` text DEFAULT NULL,
  `is_active` int(11) NOT NULL,
  `created_by` varchar(255) NOT NULL,
  PRIMARY KEY (`insulin_usage_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

CREATE TABLE `blood_sugar_log` (
  `blood_sugar_id` char(36) NOT NULL,
  `level` float NOT NULL,
  `unit` varchar(32) NOT NULL DEFAULT 'mg/dL',
  `measured_at` datetime NOT NULL,
  `meal_context` varchar(64) DEFAULT NULL,
  `notes` text DEFAULT NULL,
  `is_active` int(11) NOT NULL,
  `created_by` varchar(255) NOT NULL,
  PRIMARY KEY (`blood_sugar_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;
