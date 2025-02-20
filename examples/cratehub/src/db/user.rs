use toasty::codegen_support::*;
#[derive(Debug)]
pub struct User {
    pub id: Id<User>,
    pub name: String,
    pub email: String,
    packages: HasMany<super::package::Package>,
}
impl User {
    pub const ID: Path<Id<User>> = Path::from_field_index::<Self>(0);
    pub const NAME: Path<String> = Path::from_field_index::<Self>(1);
    pub const EMAIL: Path<String> = Path::from_field_index::<Self>(2);
    pub const PACKAGES: self::fields::Packages =
        self::fields::Packages::from_path(Path::from_field_index::<Self>(3));
    pub fn create() -> CreateUser {
        CreateUser::default()
    }
    pub fn create_many() -> CreateMany<User> {
        CreateMany::default()
    }
    pub fn filter(expr: stmt::Expr<bool>) -> Query {
        Query::from_stmt(stmt::Select::filter(expr))
    }
    pub fn update(&mut self) -> UpdateUser<'_> {
        let query = UpdateQuery::from(self.into_select());
        UpdateUser { model: self, query }
    }
    pub async fn delete(self, db: &Db) -> Result<()> {
        let stmt = self.into_select().delete();
        db.exec(stmt).await?;
        Ok(())
    }
}
impl Model for User {
    const ID: ModelId = ModelId(0);
    type Key = Id<User>;
    fn load(mut record: ValueRecord) -> Result<Self, Error> {
        Ok(User {
            id: Id::from_untyped(record[0].take().to_id()?),
            name: record[1].take().to_string()?,
            email: record[2].take().to_string()?,
            packages: HasMany::load(record[3].take())?,
        })
    }
}
impl stmt::IntoSelect for &User {
    type Model = User;
    fn into_select(self) -> stmt::Select<Self::Model> {
        User::find_by_id(&self.id).into_select()
    }
}
impl stmt::IntoSelect for &mut User {
    type Model = User;
    fn into_select(self) -> stmt::Select<Self::Model> {
        (&*self).into_select()
    }
}
impl stmt::IntoSelect for User {
    type Model = User;
    fn into_select(self) -> stmt::Select<Self::Model> {
        User::find_by_id(self.id).into_select()
    }
}
impl stmt::IntoExpr<User> for User {
    fn into_expr(self) -> stmt::Expr<User> {
        todo!()
    }
}
impl stmt::IntoExpr<User> for &User {
    fn into_expr(self) -> stmt::Expr<User> {
        stmt::Key::from_expr(&self.id).into()
    }
}
impl stmt::IntoExpr<[User]> for &User {
    fn into_expr(self) -> stmt::Expr<[User]> {
        stmt::Expr::list([self])
    }
}
#[derive(Debug)]
pub struct Query {
    stmt: stmt::Select<User>,
}
impl Query {
    pub const fn from_stmt(stmt: stmt::Select<User>) -> Query {
        Query { stmt }
    }
    pub async fn all(self, db: &Db) -> Result<Cursor<User>> {
        db.all(self.stmt).await
    }
    pub async fn first(self, db: &Db) -> Result<Option<User>> {
        db.first(self.stmt).await
    }
    pub async fn get(self, db: &Db) -> Result<User> {
        db.get(self.stmt).await
    }
    pub fn update(self) -> UpdateQuery {
        UpdateQuery::from(self)
    }
    pub async fn delete(self, db: &Db) -> Result<()> {
        db.exec(self.stmt.delete()).await?;
        Ok(())
    }
    pub async fn collect<A>(self, db: &Db) -> Result<A>
    where
        A: FromCursor<User>,
    {
        self.all(db).await?.collect().await
    }
    pub fn filter(self, expr: stmt::Expr<bool>) -> Query {
        Query {
            stmt: self.stmt.and(expr),
        }
    }
}
impl stmt::IntoSelect for Query {
    type Model = User;
    fn into_select(self) -> stmt::Select<User> {
        self.stmt
    }
}
impl stmt::IntoSelect for &Query {
    type Model = User;
    fn into_select(self) -> stmt::Select<User> {
        self.stmt.clone()
    }
}
impl Default for Query {
    fn default() -> Query {
        Query {
            stmt: stmt::Select::all(),
        }
    }
}
#[derive(Debug)]
pub struct CreateUser {
    pub(super) stmt: stmt::Insert<User>,
}
impl CreateUser {
    pub fn id(mut self, id: impl Into<Id<User>>) -> Self {
        self.stmt.set(0, id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.stmt.set(1, name.into());
        self
    }
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.stmt.set(2, email.into());
        self
    }
    pub fn package(mut self, package: impl IntoExpr<super::package::Package>) -> Self {
        self.stmt.insert(3, package.into_expr());
        self
    }
    pub async fn exec(self, db: &Db) -> Result<User> {
        db.exec_insert_one(self.stmt).await
    }
}
impl IntoInsert for CreateUser {
    type Model = User;
    fn into_insert(self) -> stmt::Insert<User> {
        self.stmt
    }
}
impl IntoExpr<User> for CreateUser {
    fn into_expr(self) -> stmt::Expr<User> {
        self.stmt.into()
    }
}
impl IntoExpr<[User]> for CreateUser {
    fn into_expr(self) -> stmt::Expr<[User]> {
        self.stmt.into_list_expr()
    }
}
impl Default for CreateUser {
    fn default() -> CreateUser {
        CreateUser {
            stmt: stmt::Insert::blank(),
        }
    }
}
#[derive(Debug)]
pub struct UpdateUser<'a> {
    model: &'a mut User,
    query: UpdateQuery,
}
#[derive(Debug)]
pub struct UpdateQuery {
    stmt: stmt::Update<User>,
}
impl UpdateUser<'_> {
    pub fn id(mut self, id: impl Into<Id<User>>) -> Self {
        self.query.set_id(id);
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.query.set_name(name);
        self
    }
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.query.set_email(email);
        self
    }
    pub fn package(mut self, package: impl IntoExpr<super::package::Package>) -> Self {
        self.query.add_package(package);
        self
    }
    pub async fn exec(self, db: &Db) -> Result<()> {
        let mut stmt = self.query.stmt;
        let mut result = db.exec_one(stmt.into()).await?;
        for (field, value) in result.into_sparse_record().into_iter() {
            match field {
                0 => self.model.id = stmt::Id::from_untyped(value.to_id()?),
                1 => self.model.name = value.to_string()?,
                2 => self.model.email = value.to_string()?,
                3 => todo!("should not be set; {} = {value:#?}", 3),
                _ => todo!("handle unknown field id in reload after update"),
            }
        }
        Ok(())
    }
}
impl UpdateQuery {
    pub fn id(mut self, id: impl Into<Id<User>>) -> Self {
        self.set_id(id);
        self
    }
    pub fn set_id(&mut self, id: impl Into<Id<User>>) -> &mut Self {
        self.stmt.set(0, id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.stmt.set(1, name.into());
        self
    }
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.set_email(email);
        self
    }
    pub fn set_email(&mut self, email: impl Into<String>) -> &mut Self {
        self.stmt.set(2, email.into());
        self
    }
    pub fn package(mut self, package: impl IntoExpr<super::package::Package>) -> Self {
        self.add_package(package);
        self
    }
    pub fn add_package(&mut self, package: impl IntoExpr<super::package::Package>) -> &mut Self {
        self.stmt.insert(3, package.into_expr());
        self
    }
    pub async fn exec(self, db: &Db) -> Result<()> {
        let stmt = self.stmt;
        let mut cursor = db.exec(stmt.into()).await?;
        Ok(())
    }
}
impl From<Query> for UpdateQuery {
    fn from(value: Query) -> UpdateQuery {
        UpdateQuery {
            stmt: stmt::Update::new(value.stmt),
        }
    }
}
impl From<stmt::Select<User>> for UpdateQuery {
    fn from(src: stmt::Select<User>) -> UpdateQuery {
        UpdateQuery {
            stmt: stmt::Update::new(src),
        }
    }
}
pub mod fields {
    use super::*;
    pub struct Packages {
        pub(super) path: Path<[super::super::package::Package]>,
    }
    impl Packages {
        pub const fn from_path(path: Path<[super::super::package::Package]>) -> Packages {
            Packages { path }
        }
        pub fn user(mut self) -> super::super::package::fields::User {
            let path = self.path.chain(super::super::package::Package::USER);
            super::super::package::fields::User::from_path(path)
        }
        pub fn user_id(mut self) -> Path<Id<User>> {
            self.path.chain(super::super::package::Package::USER_ID)
        }
        pub fn id(mut self) -> Path<Id<super::super::package::Package>> {
            self.path.chain(super::super::package::Package::ID)
        }
        pub fn name(mut self) -> Path<String> {
            self.path.chain(super::super::package::Package::NAME)
        }
    }
    impl From<Packages> for Path<[super::super::package::Package]> {
        fn from(val: Packages) -> Path<[super::super::package::Package]> {
            val.path
        }
    }
    impl<'a> stmt::IntoExpr<super::relation::packages::Packages<'a>> for Packages {
        fn into_expr(self) -> stmt::Expr<super::relation::packages::Packages<'a>> {
            todo!("into_expr for {} (field path struct)", stringify!(Packages));
        }
    }
}
pub mod relation {
    use super::*;
    use toasty::Cursor;
    pub mod packages {
        use super::*;
        #[derive(Debug)]
        pub struct Packages<'a> {
            scope: &'a User,
        }
        #[derive(Debug)]
        pub struct Query {
            pub(super) scope: super::Query,
        }
        #[derive(Debug)]
        pub struct Remove {
            stmt: stmt::Update<super::User>,
        }
        #[derive(Debug)]
        pub struct Add {
            stmt: stmt::Update<super::User>,
        }
        impl super::User {
            pub fn packages(&self) -> Packages<'_> {
                Packages { scope: self }
            }
        }
        impl super::Query {
            pub fn packages(self) -> Query {
                Query::with_scope(self)
            }
        }
        impl Packages<'_> {
            pub fn get(&self) -> &[super::super::super::package::Package] {
                self.scope.packages.get()
            }
            #[doc = r" Iterate all entries in the relation"]
            pub async fn all(
                self,
                db: &Db,
            ) -> Result<Cursor<super::super::super::package::Package>> {
                db.all(self.into_select()).await
            }
            pub async fn collect<A>(self, db: &Db) -> Result<A>
            where
                A: FromCursor<super::super::super::package::Package>,
            {
                self.all(db).await?.collect().await
            }
            #[doc = r" Create a new associated record"]
            pub fn create(self) -> super::super::super::package::CreatePackage {
                let mut builder = super::super::super::package::CreatePackage::default();
                builder.stmt.set_scope(self);
                builder
            }
            pub fn query(self, filter: stmt::Expr<bool>) -> super::super::super::package::Query {
                let query = self.into_select();
                super::super::super::package::Query::from_stmt(query.and(filter))
            }
            #[doc = r" Add an item to the association"]
            pub fn add(
                self,
                packages: impl IntoExpr<[super::super::super::package::Package]>,
            ) -> Add {
                let mut stmt = stmt::Update::new(stmt::Select::from_expr(self.scope.into_expr()));
                stmt.set_returning_none();
                stmt.insert(3, packages.into_expr());
                Add { stmt }
            }
            #[doc = r" Remove items from the association"]
            pub fn remove(
                self,
                packages: impl IntoExpr<[super::super::super::package::Package]>,
            ) -> Remove {
                let mut stmt = stmt::Update::new(stmt::Select::from_expr(self.scope.into_expr()));
                stmt.set_returning_none();
                stmt.remove(3, packages.into_expr());
                Remove { stmt }
            }
            pub fn find_by_id(
                self,
                id: impl stmt::IntoExpr<Id<super::super::super::package::Package>>,
            ) -> FindByUserAndId {
                FindByUserAndId {
                    stmt: stmt::Select::filter(
                        super::super::super::package::Package::USER
                            .in_query(self.scope)
                            .and(super::super::super::package::Package::ID.eq(id)),
                    ),
                }
            }
        }
        impl stmt::IntoSelect for Packages<'_> {
            type Model = super::super::super::package::Package;
            fn into_select(self) -> stmt::Select<super::super::super::package::Package> {
                super::super::super::package::Package::filter(
                    super::super::super::package::Package::USER.in_query(self.scope),
                )
                .into_select()
            }
        }
        impl Query {
            pub fn with_scope<S>(scope: S) -> Query
            where
                S: IntoSelect<Model = User>,
            {
                Query {
                    scope: super::Query::from_stmt(scope.into_select()),
                }
            }
            pub fn find_by_id(
                self,
                id: impl stmt::IntoExpr<Id<super::super::super::package::Package>>,
            ) -> FindByUserAndId {
                FindByUserAndId {
                    stmt: stmt::Select::filter(
                        super::super::super::package::Package::USER
                            .in_query(self.scope)
                            .and(super::super::super::package::Package::ID.eq(id)),
                    ),
                }
            }
        }
        impl Add {
            pub async fn exec(self, db: &Db) -> Result<()> {
                let mut cursor = db.exec(self.stmt.into()).await?;
                assert!(cursor.next().await.is_none());
                Ok(())
            }
        }
        impl Remove {
            pub async fn exec(self, db: &Db) -> Result<()> {
                let mut cursor = db.exec(self.stmt.into()).await?;
                assert!(cursor.next().await.is_none());
                Ok(())
            }
        }
        pub struct FindByUserAndId {
            stmt: stmt::Select<super::super::super::package::Package>,
        }
        impl FindByUserAndId {
            pub async fn all(
                self,
                db: &Db,
            ) -> Result<Cursor<super::super::super::package::Package>> {
                db.all(self.stmt).await
            }
            pub async fn first(
                self,
                db: &Db,
            ) -> Result<Option<super::super::super::package::Package>> {
                db.first(self.stmt).await
            }
            pub async fn get(self, db: &Db) -> Result<super::super::super::package::Package> {
                db.get(self.stmt).await
            }
            pub fn update(self) -> super::super::super::package::UpdateQuery {
                super::super::super::package::UpdateQuery::from(self.stmt)
            }
            pub async fn delete(self, db: &Db) -> Result<()> {
                db.exec(self.stmt.delete()).await?;
                Ok(())
            }
        }
        impl stmt::IntoSelect for FindByUserAndId {
            type Model = super::super::super::package::Package;
            fn into_select(self) -> stmt::Select<Self::Model> {
                self.stmt
            }
        }
    }
    pub use packages::Packages;
}
pub mod queries {
    use super::*;
    impl super::User {
        pub fn find_by_id(id: impl stmt::IntoExpr<Id<User>>) -> FindById {
            FindById {
                query: Query::from_stmt(stmt::Select::filter(User::ID.eq(id))),
            }
        }
    }
    pub struct FindById {
        query: Query,
    }
    impl FindById {
        pub async fn all(self, db: &Db) -> Result<Cursor<super::User>> {
            self.query.all(db).await
        }
        pub async fn first(self, db: &Db) -> Result<Option<super::User>> {
            self.query.first(db).await
        }
        pub async fn get(self, db: &Db) -> Result<super::User> {
            self.query.get(db).await
        }
        pub fn update(self) -> super::UpdateQuery {
            super::UpdateQuery::from(self.query)
        }
        pub async fn delete(self, db: &Db) -> Result<()> {
            self.query.delete(db).await
        }
        pub fn include<T: ?Sized>(mut self, path: impl Into<Path<T>>) -> FindById {
            let path = path.into();
            self.query.stmt.include(path);
            self
        }
        pub fn filter(self, filter: stmt::Expr<bool>) -> Query {
            let stmt = self.into_select();
            Query::from_stmt(stmt.and(filter))
        }
        pub async fn collect<A>(self, db: &Db) -> Result<A>
        where
            A: FromCursor<super::User>,
        {
            self.all(db).await?.collect().await
        }
        pub fn packages(mut self) -> self::relation::packages::Query {
            self::relation::packages::Query::with_scope(self)
        }
    }
    impl stmt::IntoSelect for FindById {
        type Model = super::User;
        fn into_select(self) -> stmt::Select<Self::Model> {
            self.query.into_select()
        }
    }
    impl super::User {
        pub fn find_many_by_id() -> FindManyById {
            FindManyById { items: vec![] }
        }
    }
    pub struct FindManyById {
        items: Vec<stmt::Expr<Id<User>>>,
    }
    impl FindManyById {
        pub fn item(mut self, id: impl stmt::IntoExpr<Id<User>>) -> Self {
            self.items.push(id.into_expr());
            self
        }
        pub async fn all(self, db: &Db) -> Result<Cursor<super::User>> {
            db.all(self.into_select()).await
        }
        pub async fn first(self, db: &Db) -> Result<Option<super::User>> {
            db.first(self.into_select()).await
        }
        pub async fn get(self, db: &Db) -> Result<super::User> {
            db.get(self.into_select()).await
        }
        pub fn update(self) -> super::UpdateQuery {
            super::UpdateQuery::from(self.into_select())
        }
        pub async fn delete(self, db: &Db) -> Result<()> {
            db.delete(self.into_select()).await
        }
        pub fn filter(self, filter: stmt::Expr<bool>) -> Query {
            let stmt = self.into_select();
            Query::from_stmt(stmt.and(filter))
        }
        pub async fn collect<A>(self, db: &Db) -> Result<A>
        where
            A: FromCursor<super::User>,
        {
            self.all(db).await?.collect().await
        }
    }
    impl stmt::IntoSelect for FindManyById {
        type Model = super::User;
        fn into_select(self) -> stmt::Select<Self::Model> {
            stmt::Select::filter(stmt::in_set(User::ID, self.items))
        }
    }
    impl super::User {
        pub fn find_by_email(email: impl stmt::IntoExpr<String>) -> FindByEmail {
            FindByEmail {
                query: Query::from_stmt(stmt::Select::filter(User::EMAIL.eq(email))),
            }
        }
    }
    pub struct FindByEmail {
        query: Query,
    }
    impl FindByEmail {
        pub async fn all(self, db: &Db) -> Result<Cursor<super::User>> {
            self.query.all(db).await
        }
        pub async fn first(self, db: &Db) -> Result<Option<super::User>> {
            self.query.first(db).await
        }
        pub async fn get(self, db: &Db) -> Result<super::User> {
            self.query.get(db).await
        }
        pub fn update(self) -> super::UpdateQuery {
            super::UpdateQuery::from(self.query)
        }
        pub async fn delete(self, db: &Db) -> Result<()> {
            self.query.delete(db).await
        }
        pub fn include<T: ?Sized>(mut self, path: impl Into<Path<T>>) -> FindByEmail {
            let path = path.into();
            self.query.stmt.include(path);
            self
        }
        pub fn filter(self, filter: stmt::Expr<bool>) -> Query {
            let stmt = self.into_select();
            Query::from_stmt(stmt.and(filter))
        }
        pub async fn collect<A>(self, db: &Db) -> Result<A>
        where
            A: FromCursor<super::User>,
        {
            self.all(db).await?.collect().await
        }
        pub fn packages(mut self) -> self::relation::packages::Query {
            self::relation::packages::Query::with_scope(self)
        }
    }
    impl stmt::IntoSelect for FindByEmail {
        type Model = super::User;
        fn into_select(self) -> stmt::Select<Self::Model> {
            self.query.into_select()
        }
    }
    impl super::User {
        pub fn find_many_by_email() -> FindManyByEmail {
            FindManyByEmail { items: vec![] }
        }
    }
    pub struct FindManyByEmail {
        items: Vec<stmt::Expr<String>>,
    }
    impl FindManyByEmail {
        pub fn item(mut self, email: impl stmt::IntoExpr<String>) -> Self {
            self.items.push(email.into_expr());
            self
        }
        pub async fn all(self, db: &Db) -> Result<Cursor<super::User>> {
            db.all(self.into_select()).await
        }
        pub async fn first(self, db: &Db) -> Result<Option<super::User>> {
            db.first(self.into_select()).await
        }
        pub async fn get(self, db: &Db) -> Result<super::User> {
            db.get(self.into_select()).await
        }
        pub fn update(self) -> super::UpdateQuery {
            super::UpdateQuery::from(self.into_select())
        }
        pub async fn delete(self, db: &Db) -> Result<()> {
            db.delete(self.into_select()).await
        }
        pub fn filter(self, filter: stmt::Expr<bool>) -> Query {
            let stmt = self.into_select();
            Query::from_stmt(stmt.and(filter))
        }
        pub async fn collect<A>(self, db: &Db) -> Result<A>
        where
            A: FromCursor<super::User>,
        {
            self.all(db).await?.collect().await
        }
    }
    impl stmt::IntoSelect for FindManyByEmail {
        type Model = super::User;
        fn into_select(self) -> stmt::Select<Self::Model> {
            stmt::Select::filter(stmt::in_set(User::EMAIL, self.items))
        }
    }
}
