//! This library provides an API similar to pathlib of python.
//! "/" can be used to join the path.
//! [`PathDiv`] is a wrapper for [`PathBuf`].
//!
//!
//! # Examples
//!
//! ```
//! use pathdiv::PathDiv;
//!
//! let mut path = PathDiv::from("/etc");
//! path = path / "init.d";
//! path /= "ssh";
//! println!("{}", path); // "/etc/init.d/init.d/ssh"
//! ```
//!

use std::ops;
use std::ops::Div;
use std::ops::DivAssign;
use std::path::Path;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fmt;
use std::fmt::Debug;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::cmp;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::iter;
use std::sync::Arc;
use std::rc::Rc;
use std::str::FromStr;
use std::hash::Hash;
use std::hash::Hasher;


///
/// This library provides an API similar to pathlib of python.
/// "/" can be used to join the path.
/// [`PathDiv`] is a wrapper for [`PathBuf`].
///
///
/// # Examples
///
/// ```
/// use pathdiv::PathDiv;
///
/// let mut path = PathDiv::from("/etc");
/// path = path / "init.d";
/// path /= "ssh";
/// println!("{}", path); // "/etc/init.d/init.d/ssh"
/// ```
///
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct PathDiv(PathBuf);

impl PathDiv   {

    pub fn new() -> PathDiv {
        PathDiv(PathBuf::new())
    }

    /*
    pub fn with_capacity(capacity: usize) -> PathDiv  {
        PathDiv(PathBuf::with_capacity(capacity))
    }
    */

    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }

    pub fn push<P: AsRef<Path>>(&mut self, path: P) {
        self.0.push(path);
    }

    pub fn pop(&mut self) -> bool {
        self.0.pop()
    }

    pub fn set_file_name<S: AsRef<OsStr>>(&mut self, file_name: S) {
        self.0.set_file_name(file_name);
    }

    pub fn set_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> bool {
        self.0.set_extension(extension)
    }

    pub fn into_os_string(self) -> OsString {
        self.0.into_os_string()
    }

    pub fn into_boxed_path(self) -> Box<Path> {
        self.0.into_boxed_path()
    }

    /*
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.reserve_exact(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.shrink_to(min_capacity);
    }
    */

}

impl fmt::Display for PathDiv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // self.0.fmt(f)
        f.write_str(self.0.to_str().unwrap())
    }
}

impl AsRef<OsStr> for PathDiv {
    fn as_ref(&self) -> &OsStr {
        &self.0.as_ref()
    }
}

impl AsRef<Path> for PathDiv {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl Borrow<Path> for PathDiv {
    fn borrow(&self) -> &Path {
        self.0.borrow()
    }
}

impl Default for PathDiv {
    fn default() -> Self {
        PathDiv::new()
    }
}

impl ops::Deref for PathDiv {
    type Target = Path;
    #[inline]
    fn deref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl<P: AsRef<Path>> iter::Extend<P> for PathDiv {
    fn extend<I: IntoIterator<Item = P>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl<T: ?Sized + AsRef<OsStr>> From<&T> for PathDiv {
    fn from(s: &T) -> PathDiv {
        PathDiv(PathBuf::from(s.as_ref().to_os_string()))
    }
}

impl<'a> From<&'a PathDiv> for Cow<'a, Path> {
    #[inline]
    fn from(p: &'a PathDiv) -> Cow<'a, Path> {
        Cow::Borrowed(p.as_path())
    }
}

impl From<Box<Path>> for PathDiv {
    /// Converts a `Box<Path>` into a `PathDiv`
    fn from(boxed: Box<Path>) -> PathDiv {
        PathDiv(boxed.into_path_buf())
    }
}

impl<'a> From<Cow<'a, Path>> for PathDiv {
    #[inline]
    fn from(p: Cow<'a, Path>) -> Self {
        PathDiv(p.into_owned())
    }
}

impl From<OsString> for PathDiv {
    /// Converts a `OsString` into a `PathDiv`
    #[inline]
    fn from(s: OsString) -> PathDiv {
        PathDiv(PathBuf::from(s))
    }
}

impl From<PathDiv> for Box<Path> {
    /// Converts a `PathDiv` into a `Box<Path>`
    fn from(p: PathDiv) -> Box<Path> {
        p.into_boxed_path()
    }
}

impl From<PathDiv> for OsString {
    /// Converts a `PathDiv` into a `OsString`
    fn from(path_buf: PathDiv) -> OsString {
        path_buf.0.as_os_str().to_os_string()
    }
}

/*
impl<'a> From<PathDiv> for Cow<'a, Path> {
    #[inline]
    fn from(s: PathDiv) -> Cow<'a, Path> {
        Cow::Owned(s)
    }
}
*/

impl From<PathDiv> for Arc<Path> {
    /// Converts a `PathDiv` into an `Arc` by moving the `PathBuf` data into a new `Arc` buffer.
    #[inline]
    fn from(s: PathDiv) -> Arc<Path> {
        let arc: Arc<OsStr> = Arc::from(s.into_os_string());
        unsafe { Arc::from_raw(Arc::into_raw(arc) as *const Path) }
    }
}

impl From<PathDiv> for Rc<Path> {
    /// Converts a `PathBuf` into an `Rc` by moving the `PathBuf` data into a new `Rc` buffer.
    #[inline]
    fn from(s: PathDiv) -> Rc<Path> {
        let rc: Rc<OsStr> = Rc::from(s.into_os_string());
        unsafe { Rc::from_raw(Rc::into_raw(rc) as *const Path) }
    }
}

impl From<String> for PathDiv {
    /// Converts a `String` into a `PathBuf`
    fn from(s: String) -> PathDiv {
        PathDiv(PathBuf::from(OsString::from(s)))
    }
}

impl<P: AsRef<Path>> iter::FromIterator<P> for PathDiv {
    fn from_iter<I: IntoIterator<Item = P>>(iter: I) -> PathDiv {
        let mut buf = PathDiv::new();
        buf.extend(iter);
        buf
    }
}

impl FromStr for PathDiv {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PathDiv::from(s))
    }
}

impl Hash for PathDiv {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.as_path().hash(h)
    }
}

/*
impl<'a> IntoIterator for &'a PathDiv {
    type Item = &'a OsStr;
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}
*/

impl cmp::Ord for PathDiv {
    fn cmp(&self, other: &PathDiv) -> cmp::Ordering {
        self.components().cmp(other.components())
    }
}

macro_rules! impl_cmp_os_str {
    ($lhs:ty, $rhs: ty) => {
        // #[stable(feature = "cmp_path", since = "1.8.0")]
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <Path as PartialEq>::eq(self, other.as_ref())
            }
        }

        // #[stable(feature = "cmp_path", since = "1.8.0")]
        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <Path as PartialEq>::eq(self.as_ref(), other)
            }
        }

        // #[stable(feature = "cmp_path", since = "1.8.0")]
        impl<'a, 'b> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<cmp::Ordering> {
                <Path as PartialOrd>::partial_cmp(self, other.as_ref())
            }
        }

        // #[stable(feature = "cmp_path", since = "1.8.0")]
        impl<'a, 'b> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<cmp::Ordering> {
                <Path as PartialOrd>::partial_cmp(self.as_ref(), other)
            }
        }
    };
}

impl_cmp_os_str!(PathDiv, OsStr);
impl_cmp_os_str!(PathDiv, &'a OsStr);
impl_cmp_os_str!(PathDiv, Cow<'a, OsStr>);
impl_cmp_os_str!(PathDiv, OsString);


macro_rules! impl_cmp {
    ($lhs:ty, $rhs: ty) => {
        // #[stable(feature = "partialeq_path", since = "1.6.0")]
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <Path as PartialEq>::eq(self, other)
            }
        }

        // #[stable(feature = "partialeq_path", since = "1.6.0")]
        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <Path as PartialEq>::eq(self, other)
            }
        }

        // #[stable(feature = "cmp_path", since = "1.8.0")]
        impl<'a, 'b> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<cmp::Ordering> {
                <Path as PartialOrd>::partial_cmp(self, other)
            }
        }

        // #[stable(feature = "cmp_path", since = "1.8.0")]
        impl<'a, 'b> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<cmp::Ordering> {
                <Path as PartialOrd>::partial_cmp(self, other)
            }
        }
    };
}

impl_cmp!(PathDiv, Path);
impl_cmp!(PathDiv, &'a Path);
impl_cmp!(Cow<'a, Path>, PathDiv);


impl<T> Div<T> for PathDiv
where
    T: AsRef<Path> + Sized
{
    type Output = Self;

    fn div(mut self, rhs: T) -> Self::Output {
        self.0.push(rhs);
        self
    }
}

impl<T> Div<T> for &PathDiv
where
    T: AsRef<Path> + Sized
{
    type Output = PathDiv ;

    fn div(self, rhs: T) -> Self::Output {
        let mut pbuf = self.0.clone();
        pbuf.push(rhs);
        PathDiv (pbuf)
    }
}

impl<T> DivAssign<T> for PathDiv
where
    T: AsRef<Path> + Sized
{
    fn div_assign(&mut self, rhs: T) {
        self.0.push(rhs);
    }
}
