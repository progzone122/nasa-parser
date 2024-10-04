CREATE TABLE `posts` (
                         `id` int(11) NOT NULL AUTO_INCREMENT,
                         `html` text NOT NULL,
                         `text` text NOT NULL,
                         PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `news` (
                        `id` int(11) NOT NULL AUTO_INCREMENT,
                        `title` varchar(22) NOT NULL,
                        `short_description` varchar(22) DEFAULT NULL,
                        `image` varchar(44) DEFAULT NULL,
                        `url` varchar(44) NOT NULL,
                        `post_id` int(11) NOT NULL,
                        PRIMARY KEY (`id`),
                        UNIQUE KEY `post_id` (`post_id`),
                        UNIQUE KEY `url` (`url`),
                        CONSTRAINT `news_ibfk_1` FOREIGN KEY (`post_id`) REFERENCES `posts` (`id`) ON DELETE NO ACTION ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
