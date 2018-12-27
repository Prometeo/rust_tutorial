struct Ref<'a, T: 'a>(&'a T);
// adding an explicit lifetime bound `T: 'a` so that the reference type
// `&'a T` does not outlive the data it points at


struct StaticRef<T: 'static>(&'static T);
// We could solve this problem in a different way, as shown in the
// definition of a StaticRef struct in Listing 19-18, by adding the
// 'static lifetime bound on T. This means if T contains any references,
// they must have the 'static lifetime
