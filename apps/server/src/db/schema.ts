import { index, integer, pgTable, text, timestamp, unique } from "drizzle-orm/pg-core";

export const users = pgTable("users", {
  id: text("id").primaryKey(),
  username: text("username").notNull().unique(),
  passwordHash: text("password_hash").notNull(),
  walletBalance: integer("wallet_balance").notNull().default(500),
  createdAt: timestamp("created_at", { withTimezone: true }).notNull().defaultNow(),
});

export const items = pgTable("items", {
  id: text("id").primaryKey(),
  slug: text("slug").notNull().unique(),
  name: text("name").notNull(),
  description: text("description").notNull(),
  price: integer("price").notNull().default(0),
});

export const inventory = pgTable(
  "inventory",
  {
    id: text("id").primaryKey(),
    userId: text("user_id")
      .notNull()
      .references(() => users.id, { onDelete: "cascade" }),
    itemId: text("item_id")
      .notNull()
      .references(() => items.id, { onDelete: "cascade" }),
    quantity: integer("quantity").notNull().default(1),
  },
  (table) => [unique("inventory_user_item_unique").on(table.userId, table.itemId)],
);

export const forumPosts = pgTable(
  "forum_posts",
  {
    id: text("id").primaryKey(),
    authorId: text("author_id")
      .notNull()
      .references(() => users.id, { onDelete: "cascade" }),
    title: text("title").notNull(),
    body: text("body").notNull(),
    createdAt: timestamp("created_at", { withTimezone: true }).notNull().defaultNow(),
  },
  (table) => [index("forum_posts_created_at_idx").on(table.createdAt)],
);

export type UserRow = typeof users.$inferSelect;
export type NewUserRow = typeof users.$inferInsert;
export type ForumPostRow = typeof forumPosts.$inferSelect;